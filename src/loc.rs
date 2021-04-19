use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::Deserialize;
use std::cell::{Cell, RefCell};
use std::fmt::{self, Debug};

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SourceRange {
    pub begin: SourceLocation,
    pub end: SourceLocation,
}

#[derive(Default)]
pub struct SourceLocation {
    pub spelling_loc: Option<BareSourceLocation>,
    pub expansion_loc: Option<BareSourceLocation>,
}

#[derive(Clone, Debug)]
pub struct BareSourceLocation {
    pub offset: usize,
    pub file: String,
    pub line: usize,
    pub presumed_file: Option<String>,
    pub presumed_line: Option<usize>,
    pub col: usize,
    pub tok_len: usize,
    pub included_from: Option<IncludedFrom>,
    pub is_macro_arg_expansion: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IncludedFrom {
    #[serde(rename = "includedFrom", skip_deserializing)]
    pub included_from: Option<Box<IncludedFrom>>,
    pub file: String,
}

thread_local! {
    static LAST_LOC_FILENAME: RefCell<String> = RefCell::new(String::new());
    static LAST_LOC_LINE: Cell<usize> = Cell::new(0);
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "camelCase")]
enum SourceLocationField {
    SpellingLoc,
    ExpansionLoc,
    Offset,
    File,
    Line,
    PresumedFile,
    PresumedLine,
    Col,
    TokLen,
    IncludedFrom,
    IsMacroArgExpansion,
}

impl<'de> Deserialize<'de> for SourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SourceLocationVisitor;

        impl<'de> Visitor<'de> for SourceLocationVisitor {
            type Value = SourceLocation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SourceLocation")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                match map.next_key()? {
                    None => Ok(SourceLocation::default()),
                    Some(SourceLocationField::SpellingLoc) => {
                        let spelling_loc: BareSourceLocation = map.next_value()?;
                        match map.next_key()? {
                            None => Err(Error::missing_field("expansionLoc")),
                            Some(SourceLocationField::ExpansionLoc) => {
                                let expansion_loc: BareSourceLocation = map.next_value()?;
                                Ok(SourceLocation {
                                    spelling_loc: Some(spelling_loc),
                                    expansion_loc: Some(expansion_loc),
                                })
                            }
                            Some(other) => Err(other.unexpected()),
                        }
                    }
                    Some(SourceLocationField::Offset) => {
                        let loc = de_rest_of_bare_source_location(map)?;
                        Ok(SourceLocation {
                            spelling_loc: Some(loc.clone()),
                            expansion_loc: Some(loc),
                        })
                    }
                    Some(other) => Err(other.unexpected()),
                }
            }
        }

        deserializer.deserialize_map(SourceLocationVisitor)
    }
}

impl<'de> Deserialize<'de> for BareSourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BareSourceLocationVisitor;

        impl<'de> Visitor<'de> for BareSourceLocationVisitor {
            type Value = BareSourceLocation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BareSourceLocation")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                match map.next_key()? {
                    None => Err(Error::missing_field("offset")),
                    Some(SourceLocationField::Offset) => de_rest_of_bare_source_location(map),
                    Some(other) => Err(other.unexpected()),
                }
            }
        }

        deserializer.deserialize_map(BareSourceLocationVisitor)
    }
}

fn de_rest_of_bare_source_location<'de, M>(mut map: M) -> Result<BareSourceLocation, M::Error>
where
    M: MapAccess<'de>,
{
    let offset: usize = map.next_value()?;

    let mut file = None::<String>;
    let mut line = None;
    let mut presumed_file = None;
    let mut presumed_line = None;
    let mut col = None;
    let mut tok_len = None;
    let mut included_from = None;
    let mut is_macro_arg_expansion = false;

    while let Some(field) = map.next_key()? {
        match field {
            SourceLocationField::Offset => return Err(Error::duplicate_field("offset")),
            SourceLocationField::File => file = Some(map.next_value()?),
            SourceLocationField::Line => line = Some(map.next_value()?),
            SourceLocationField::PresumedFile => {
                presumed_file = Some(map.next_value()?);
            }
            SourceLocationField::PresumedLine => presumed_line = Some(map.next_value()?),
            SourceLocationField::Col => col = Some(map.next_value()?),
            SourceLocationField::TokLen => tok_len = Some(map.next_value()?),
            SourceLocationField::IncludedFrom => included_from = Some(map.next_value()?),
            SourceLocationField::IsMacroArgExpansion => {
                is_macro_arg_expansion = map.next_value()?;
            }
            SourceLocationField::SpellingLoc | SourceLocationField::ExpansionLoc => {
                return Err(field.unexpected());
            }
        }
    }

    let file = LAST_LOC_FILENAME.with(|last_loc_filename| match file {
        Some(file) => {
            *last_loc_filename.borrow_mut() = file.clone();
            file
        }
        None => last_loc_filename.borrow().clone(),
    });

    let line = LAST_LOC_LINE.with(|last_loc_line| match line {
        Some(line) => {
            last_loc_line.set(line);
            line
        }
        None => last_loc_line.get(),
    });

    let col = col.ok_or_else(|| Error::missing_field("col"))?;
    let tok_len = tok_len.ok_or_else(|| Error::missing_field("tokLen"))?;

    Ok(BareSourceLocation {
        offset,
        file,
        line,
        presumed_file,
        presumed_line,
        col,
        tok_len,
        included_from,
        is_macro_arg_expansion,
    })
}

impl SourceLocationField {
    fn unexpected<E: Error>(&self) -> E {
        Error::unknown_field(
            match self {
                SourceLocationField::SpellingLoc => "spellingLoc",
                SourceLocationField::ExpansionLoc => "expansionLoc",
                SourceLocationField::Offset => "offset",
                SourceLocationField::File => "file",
                SourceLocationField::Line => "line",
                SourceLocationField::PresumedFile => "presumedFile",
                SourceLocationField::PresumedLine => "presumedLine",
                SourceLocationField::Col => "col",
                SourceLocationField::TokLen => "tokLen",
                SourceLocationField::IncludedFrom => "includedFrom",
                SourceLocationField::IsMacroArgExpansion => "isMacroArgExpansion",
            },
            &[
                "spellingLoc",
                "expansionLoc",
                "offset",
                "file",
                "line",
                "presumedFile",
                "presumedLine",
                "col",
                "tokLen",
                "includedFrom",
                "isMacroArgExpansion",
            ],
        )
    }
}

impl Debug for SourceRange {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let SourceRange { begin, end } = self;
        let SourceLocation {
            spelling_loc: begin_spelling_loc,
            expansion_loc: begin_expansion_loc,
        } = begin;
        let SourceLocation {
            spelling_loc: end_spelling_loc,
            expansion_loc: end_expansion_loc,
        } = end;
        let mut debug = formatter.debug_struct("SourceRange");
        if begin_spelling_loc.is_some()
            || begin_expansion_loc.is_some()
            || end_spelling_loc.is_some()
            || end_expansion_loc.is_some()
        {
            debug.field("begin", begin).field("end", end);
        }
        debug.finish()
    }
}

impl Debug for SourceLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let SourceLocation {
            spelling_loc,
            expansion_loc,
        } = self;
        let mut debug = formatter.debug_struct("SourceLocation");
        if spelling_loc.is_some() {
            debug.field("spelling_loc", spelling_loc);
        }
        if expansion_loc.is_some() {
            debug.field("expansion_loc", expansion_loc);
        }
        debug.finish()
    }
}
