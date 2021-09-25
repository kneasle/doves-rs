use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use serde::{de::Error, Deserialize, Deserializer};

/// A list of towers read from Dove's Guide.
#[derive(Debug, Clone)]
pub struct Doves {
    towers: Vec<Ring>,
}

/// A `Ring` of bells in Dove's Guide.  Note that the same tower could contain multiple `Ring`s.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ring {
    /// The Dove's tower ID.  This is unique for each tower and will never change between updates
    /// to Dove's Guide.
    ///
    /// **CSV Header**: `TowerID`
    #[serde(rename = "TowerID")]
    pub id: usize,
    /// What type of ring this is.
    ///
    /// **CSV Header**: `RingType`
    #[serde(rename = "RingType")]
    pub ring_type: RingType,
    #[serde(rename = "Bells")]
    pub bells: usize,
    /// `true` if the set of bells cannot be safely rung.
    ///
    /// **CSV Header**: `UR`; `""` if `false`, `"u/r"` if `true`
    #[serde(rename = "UR", deserialize_with = "deser_not_empty")]
    pub unringable: bool,
    /// `true` if the set of bells are rung from the floor.
    ///
    /// **CSV Header**: `GF`; `""` if `false`, `"GF"` if `true`
    #[serde(rename = "GF", deserialize_with = "deser_not_empty")]
    pub ground_floor: bool,
    /// `true` if the church has toilet facilities.
    ///
    /// **CSV Header**: `Toilet`; `""` if `false`, `"T"` if `true`
    #[serde(rename = "Toilet", deserialize_with = "deser_not_empty")]
    pub toilet: bool,
    /// `true` if the bells can be rung without making a sound using a simulator.
    ///
    /// **CSV Header**: `Toilet`; `""` if `false`, `"T"` if `true`
    #[serde(rename = "Simulator", deserialize_with = "deser_not_empty")]
    pub simulator: bool,
    /// Set of [`Affiliation`]s to which this tower belongs.
    ///
    /// **CSV Header**: `Affiliations`
    #[serde(rename = "Affiliations", deserialize_with = "deser_affiliations")]
    pub affiliations: HashSet<Affiliation>,
    /// String describing when this tower holds a practice night.
    ///
    /// **CSV Header**: `Practice`
    #[serde(rename = "Practice")]
    pub practice: Option<String>,

    /// Identifier for this tower used in TowerBase.  These are not unique between every [`Tower`]
    /// object.
    ///
    /// **CSV Header**: `Practice`
    #[serde(rename = "TowerBase")]
    pub towerbase_id: usize,
    /// Unique text identifier for towers.  Doves have deprecated this; use the `id` field instead.
    ///
    /// **CSV Header**: `Practice`
    #[serde(rename = "DoveID")]
    #[deprecated(note = "please use `id` instead")]
    pub dove_id: Option<String>, // PERF: Use SmolStr here to prevent allocs

    /// The [`Weight`] of the heaviest bell in this `Ring`.
    ///
    /// **CSV Header**: `Wt`
    #[serde(rename = "Wt", deserialize_with = "deser_weight")]
    pub weight: Weight,
    /// The [`Note`] of the heaviest bell in this `Ring`.
    ///
    /// **CSV Header**: `Note`
    #[serde(rename = "Note", deserialize_with = "deser_option_note")]
    pub note: Option<Note>,
    /// The frequency (in Hz) of the heaviest bell in this `Ring`.
    ///
    /// **CSV Header**: `Hz`
    #[serde(rename = "Hz")]
    pub freq: Option<f64>, // TODO: Is this linked with `note`

    /// Details about this ring.  This is either `"P"` or `"C"`, and I don't know what this means.
    ///
    /// **CSV Header**: `Details`
    #[serde(rename = "Details")]
    pub details: Details,
    /// List of extra pieces of information about this `Ring`.
    ///
    /// **CSV Header**: `ExtraInfo`, semicolon-delimited list
    #[serde(rename = "ExtraInfo", deserialize_with = "deser_semicolon_list")]
    pub extra_info: Vec<String>,
    /// URL to the web-page of this `Ring`
    ///
    /// **CSV Header**: `WebPage`
    #[serde(rename = "WebPage")]
    pub url: Option<String>,

    /// '+'-delimited list of semitones bells in this `Ring`.
    ///
    /// **CSV Header**: `Semitones`, '+'-delimited list
    #[serde(rename = "Semitones")]
    pub semitones: Option<String>, // TODO: Parse this into a struct
    /// TODO: What does this do?
    ///
    /// **CSV Header**: `App`; `""` for `false`, `"app"` for `true`.
    #[serde(rename = "App", deserialize_with = "deser_not_empty")]
    pub app: bool,

    // TODO: Figure out what values are legal
    /// The name of the place which contains this `Ring`.  If the ring is mobile, then this is set
    /// to the name of the `Ring`.
    ///
    /// **CSV Header**: `Place`
    #[serde(rename = "Place")]
    pub place: String,
    /// An alternative place name for this `Ring`, more specific than `place`.
    ///
    /// **CSV Header**: `Place2`
    #[serde(rename = "Place2")]
    pub place2: Option<String>,
    /// The name of this `Ring` used in county lists.
    ///
    /// **CSV Header**: `PlaceCL`
    #[serde(rename = "PlaceCL")]
    pub place_county_list: Option<String>,
    #[serde(rename = "County")]
    pub county: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
    #[serde(rename = "ISO3166code")]
    pub iso_3166_code: Option<String>,
    #[serde(rename = "NG")]
    pub os_grid_ref: Option<String>, // TODO: Parse this into a struct?
    #[serde(rename = "Postcode")]
    pub postcode: Option<String>, // TODO: Parse this into a struct?

    #[serde(rename = "Long")]
    pub long: Option<f64>,
    #[serde(rename = "Lat")]
    pub lat: Option<f64>,

    // TODO: What are these for?
    #[serde(rename = "SNLong")]
    pub satnav_long: Option<f64>,
    #[serde(rename = "SNLat")]
    pub satnav_lat: Option<f64>,

    #[serde(rename = "OvhaulYr")]
    pub overhaul_year: Option<usize>,
    #[serde(rename = "Contractor")]
    pub contractor: Option<String>, // TODO: Is this linked to `overhaul_yr`
    #[serde(rename = "TuneYr")]
    pub tune_year: Option<usize>,

    #[serde(rename = "BldgID")]
    pub building_id: Option<usize>, // TODO: Can this be empty
    #[serde(rename = "LGrade")]
    pub building_grade: Option<String>, // TODO: Make a struct for this?

    #[serde(rename = "ChurchCare")]
    pub church_care: Option<usize>, // TODO: What is this?

    #[serde(rename = "Dedicn")]
    pub dedication: String, // TODO: Is this optional?
    #[serde(rename = "AltName")]
    pub alt_name: Option<String>,
    #[serde(rename = "Diocese")]
    pub diocese: Option<String>, // TODO: Could this be an enum?
}

/// The possible types of a rings documented in Dove's Guide.
#[derive(Debug, Clone, Deserialize)]
#[serde(field_identifier)]
pub enum RingType {
    #[serde(rename = "Full circle ring")]
    FullCircle,
    Carillon,
}

/// The `Weight` of the heaviest bell in a [`Ring`]
///
/// TODO: Add more methods for this
#[derive(Debug, Clone)]
pub struct Weight {
    lbs: f64,
}

/// An organisation to which a tower can be affiliated.
///
/// TODO: Make this a bit-field.
/// TODO: Populate this.  Options are:
/// {'', 'GDG', 'W&P', 'HCA', 'HDG', 'SMB', 'CEA', 'YACR', 'Suff',
/// 'EDWNA', 'LinDG', 'L&M', 'NDA', 'Essex', 'LeiDG', 'UBSCR', 'Ely', 'Lancs', 'TruDG', 'Beds',
/// 'G&B', 'OS', 'Swell', 'B&W', 'Lundy', 'CarDG', 'LivUS', 'SuxCA', 'NSA', 'EGDG', 'ECBA',
/// 'ANZAB', 'DDA', 'Salis', 'ULSCR', 'PDG', 'Bev&D', 'NWA', 'CovDG', 'Irish', 'Middx',
/// 'Salop', 'CheDG', 'D&N', 'WDA', 'NAG', 'DCA', 'SAG', 'SRCY', 'ASCY', 'DevAs', 'SDDG', 'KCA',
/// 'Zimb', 'Scot', 'GDR', 'S&B', 'LWAS'}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Affiliation {
    /* UNIVERSITY SOCIETIES */
    /// Cambridge University Guild
    ///
    /// **Dove's value:** `CUG`
    CambridgeUni,
    /// Manchester University Guild
    ///
    /// **Dove's value:** `MUG`
    ManchesterUni,
    /// Oxford University Society
    ///
    /// **Dove's value:** `OUS`
    OxfordUni,

    /* GEOGRAPHIC ASSOCIATIONS */
    /// Oxford Diocesian Guild
    ///
    /// **Dove's value:** `ODG`
    OxfordDiocese,
    /// Surrey Association
    ///
    /// **Dove's value:** `Surr`
    Surrey,
}

/// Details related to any given tower.  I have no idea what this means.  TODO: Find out
#[derive(Debug, Clone, Deserialize)]
#[serde(field_identifier)]
pub enum Details {
    P,
    C,
}

///////////
// NOTES //
///////////

const FLAT: char = '♭';
const SHARP: char = '♯';

/// The `Weight` of the heaviest bell in a [`Ring`]
#[derive(Debug, Clone)]
pub struct Note {
    pub name: NoteName,
    pub accidental: Accidental,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name, self.accidental)
    }
}

/// The name of a root [`Note`] (i.e. `A` to `G`)
#[derive(Debug, Clone)]
pub enum NoteName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Display using the note names
    }
}

/// An accidental to be applied to a [`NoteName`] to create a specific [`Note`].
#[derive(Debug, Clone)]
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Accidental::Flat => write!(f, "{}", FLAT),
            Accidental::Natural => Ok(()),
            Accidental::Sharp => write!(f, "{}", SHARP),
        }
    }
}

/////////////////////////////
// DESERIALIZATION HELPERS //
/////////////////////////////

/// Serializes as `false` if the next string is empty and `true` otherwise.
fn deser_not_empty<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    <&str>::deserialize(de).map(str::is_empty)
}

/// Serializes the next string as a `;`-delimited sequence of [`Affiliation`]s
fn deser_affiliations<'de, D>(de: D) -> Result<HashSet<Affiliation>, D::Error>
where
    D: Deserializer<'de>,
{
    todo!()
}

/// Serializes the next string as a `;`-delimited sequence of strings
fn deser_semicolon_list<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    <&str>::deserialize(de).map(|s| s.split(';').map(str::to_owned).collect())
}

/// Serializes the next string as a weight in pounds
fn deser_weight<'de, D>(de: D) -> Result<Weight, D::Error>
where
    D: Deserializer<'de>,
{
    f64::deserialize(de).map(|lbs| Weight { lbs })
}

/// Serializes the next string as a note, or `None` if the string is empty
fn deser_option_note<'de, D>(de: D) -> Result<Option<Note>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str>::deserialize(de)?;
    let mut chars = s.chars();
    let name = match chars.next() {
        None => return Ok(None), // Empty strings correspond to `None`
        Some('A') => NoteName::A,
        Some('B') => NoteName::B,
        Some('C') => NoteName::C,
        Some('D') => NoteName::D,
        Some('E') => NoteName::E,
        Some('F') => NoteName::F,
        Some('G') => NoteName::G,
        Some(c) => return Err(D::Error::custom(format!("Char {:?} is not a note name", c))),
    };
    let accidental = match chars.next() {
        Some('♭') | Some('b') => Accidental::Flat,
        Some('♮') | None => Accidental::Natural,
        Some('♯') | Some('#') => Accidental::Sharp,
        Some(c) => {
            return Err(D::Error::custom(format!(
                "Char {:?} is not an accidental name",
                c
            )))
        }
    };
    Ok(Some(Note { name, accidental }))
}
