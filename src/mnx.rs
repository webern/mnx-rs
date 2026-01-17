/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///BeamList
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "array",
///  "items": {
///    "type": "object",
///    "required": [
///      "events"
///    ],
///    "properties": {
///      "events": {
///        "type": "array",
///        "items": {
///          "$ref": "#/$defs/id"
///        }
///      },
///      "hooks": {
///        "type": "array",
///        "items": {
///          "type": "object",
///          "required": [
///            "direction",
///            "event"
///          ],
///          "properties": {
///            "direction": {
///              "type": "string",
///              "enum": [
///                "left",
///                "right"
///              ]
///            },
///            "event": {
///              "$ref": "#/$defs/id"
///            }
///          },
///          "additionalProperties": false
///        }
///      },
///      "inner": {
///        "$ref": "#/$defs/beam-list"
///      }
///    },
///    "additionalProperties": false
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct BeamList(pub ::std::vec::Vec<BeamListItem>);
impl ::std::ops::Deref for BeamList {
    type Target = ::std::vec::Vec<BeamListItem>;
    fn deref(&self) -> &::std::vec::Vec<BeamListItem> {
        &self.0
    }
}
impl ::std::convert::From<BeamList> for ::std::vec::Vec<BeamListItem> {
    fn from(value: BeamList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BeamList> for BeamList {
    fn from(value: &BeamList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<BeamListItem>> for BeamList {
    fn from(value: ::std::vec::Vec<BeamListItem>) -> Self {
        Self(value)
    }
}
///BeamListItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "events"
///  ],
///  "properties": {
///    "events": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/id"
///      }
///    },
///    "hooks": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "direction",
///          "event"
///        ],
///        "properties": {
///          "direction": {
///            "type": "string",
///            "enum": [
///              "left",
///              "right"
///            ]
///          },
///          "event": {
///            "$ref": "#/$defs/id"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "inner": {
///      "$ref": "#/$defs/beam-list"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BeamListItem {
    pub events: ::std::vec::Vec<Id>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hooks: ::std::vec::Vec<BeamListItemHooksItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inner: ::std::option::Option<BeamList>,
}
impl ::std::convert::From<&BeamListItem> for BeamListItem {
    fn from(value: &BeamListItem) -> Self {
        value.clone()
    }
}
impl BeamListItem {
    pub fn builder() -> builder::BeamListItem {
        Default::default()
    }
}
///BeamListItemHooksItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "event"
///  ],
///  "properties": {
///    "direction": {
///      "type": "string",
///      "enum": [
///        "left",
///        "right"
///      ]
///    },
///    "event": {
///      "$ref": "#/$defs/id"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BeamListItemHooksItem {
    pub direction: BeamListItemHooksItemDirection,
    pub event: Id,
}
impl ::std::convert::From<&BeamListItemHooksItem> for BeamListItemHooksItem {
    fn from(value: &BeamListItemHooksItem) -> Self {
        value.clone()
    }
}
impl BeamListItemHooksItem {
    pub fn builder() -> builder::BeamListItemHooksItem {
        Default::default()
    }
}
///BeamListItemHooksItemDirection
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "left",
///    "right"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BeamListItemHooksItemDirection {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}
impl ::std::convert::From<&Self> for BeamListItemHooksItemDirection {
    fn from(value: &BeamListItemHooksItemDirection) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BeamListItemHooksItemDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}
impl ::std::str::FromStr for BeamListItemHooksItemDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BeamListItemHooksItemDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BeamListItemHooksItemDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BeamListItemHooksItemDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Color
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct Color(pub ::std::string::String);
impl ::std::ops::Deref for Color {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Color> for ::std::string::String {
    fn from(value: Color) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Color> for Color {
    fn from(value: &Color) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Color {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Color {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///Event
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "duration": {
///      "$ref": "#/$defs/note-value"
///    },
///    "id": {
///      "$ref": "#/$defs/id"
///    },
///    "lyrics": {
///      "type": "object",
///      "properties": {
///        "lines": {
///          "type": "object",
///          "patternProperties": {
///            "^.*$": {
///              "type": "object",
///              "required": [
///                "text"
///              ],
///              "properties": {
///                "text": {
///                  "$ref": "#/$defs/string"
///                },
///                "type": {
///                  "type": "string",
///                  "enum": [
///                    "start",
///                    "middle",
///                    "end",
///                    "whole"
///                  ]
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "additionalProperties": false
///        }
///      },
///      "additionalProperties": false
///    },
///    "markings": {
///      "type": "object",
///      "properties": {
///        "accent": {
///          "type": "object",
///          "properties": {
///            "pointing": {
///              "$ref": "#/$defs/up-or-down"
///            }
///          },
///          "additionalProperties": false
///        },
///        "breath": {
///          "type": "object",
///          "properties": {
///            "symbol": {
///              "type": "string"
///            }
///          },
///          "additionalProperties": false
///        },
///        "softAccent": {
///          "type": "object",
///          "additionalProperties": false
///        },
///        "spiccato": {
///          "type": "object",
///          "additionalProperties": false
///        },
///        "staccatissimo": {
///          "type": "object",
///          "additionalProperties": false
///        },
///        "staccato": {
///          "type": "object",
///          "additionalProperties": false
///        },
///        "stress": {
///          "type": "object",
///          "additionalProperties": false
///        },
///        "strongAccent": {
///          "type": "object",
///          "properties": {
///            "pointing": {
///              "$ref": "#/$defs/up-or-down"
///            }
///          },
///          "additionalProperties": false
///        },
///        "tenuto": {
///          "type": "object",
///          "additionalProperties": false
///        },
///        "tremolo": {
///          "type": "object",
///          "required": [
///            "marks"
///          ],
///          "properties": {
///            "marks": {
///              "$ref": "#/$defs/positive-integer"
///            }
///          },
///          "additionalProperties": false
///        },
///        "unstress": {
///          "type": "object",
///          "additionalProperties": false
///        }
///      },
///      "additionalProperties": false
///    },
///    "measure": {
///      "type": "boolean"
///    },
///    "notes": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "pitch"
///        ],
///        "properties": {
///          "accidentalDisplay": {
///            "type": "object",
///            "required": [
///              "show"
///            ],
///            "properties": {
///              "enclosure": {
///                "type": "object",
///                "required": [
///                  "symbol"
///                ],
///                "properties": {
///                  "symbol": {
///                    "type": "string",
///                    "enum": [
///                      "parentheses",
///                      "brackets"
///                    ]
///                  }
///                },
///                "additionalProperties": false
///              },
///              "force": {
///                "type": "boolean"
///              },
///              "show": {
///                "type": "boolean"
///              }
///            },
///            "additionalProperties": false
///          },
///          "class": {
///            "$ref": "#/$defs/style-class"
///          },
///          "id": {
///            "$ref": "#/$defs/id"
///          },
///          "perform": {
///            "type": "object",
///            "additionalProperties": false
///          },
///          "pitch": {
///            "type": "object",
///            "required": [
///              "octave",
///              "step"
///            ],
///            "properties": {
///              "alter": {
///                "type": "integer"
///              },
///              "octave": {
///                "type": "integer"
///              },
///              "step": {
///                "type": "string",
///                "enum": [
///                  "A",
///                  "B",
///                  "C",
///                  "D",
///                  "E",
///                  "F",
///                  "G"
///                ]
///              }
///            },
///            "additionalProperties": false
///          },
///          "smuflFont": {
///            "$ref": "#/$defs/smufl-font"
///          },
///          "staff": {
///            "$ref": "#/$defs/staff-number"
///          },
///          "ties": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "lv": {
///                  "type": "boolean"
///                },
///                "side": {
///                  "$ref": "#/$defs/slur-side"
///                },
///                "target": {
///                  "$ref": "#/$defs/id"
///                }
///              },
///              "additionalProperties": false
///            }
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "orient": {
///      "$ref": "#/$defs/orientation"
///    },
///    "rest": {
///      "type": "object",
///      "properties": {
///        "staffPosition": {
///          "$ref": "#/$defs/staff-position"
///        }
///      },
///      "additionalProperties": false
///    },
///    "slurs": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "target"
///        ],
///        "properties": {
///          "endNote": {
///            "$ref": "#/$defs/id"
///          },
///          "lineType": {
///            "type": "string",
///            "enum": [
///              "dashed",
///              "dotted",
///              "solid",
///              "wavy"
///            ]
///          },
///          "side": {
///            "$ref": "#/$defs/slur-side"
///          },
///          "sideEnd": {
///            "$ref": "#/$defs/slur-side"
///          },
///          "startNote": {
///            "$ref": "#/$defs/id"
///          },
///          "target": {
///            "$ref": "#/$defs/id"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "smuflFont": {
///      "$ref": "#/$defs/smufl-font"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    },
///    "stemDirection": {
///      "$ref": "#/$defs/stem-direction"
///    },
///    "type": {
///      "type": "string",
///      "const": "event"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: ::std::option::Option<NoteValue>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lyrics: ::std::option::Option<EventLyrics>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub markings: ::std::option::Option<EventMarkings>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub measure: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub notes: ::std::vec::Vec<EventNotesItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orient: ::std::option::Option<Orientation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rest: ::std::option::Option<EventRest>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub slurs: ::std::vec::Vec<EventSlursItem>,
    #[serde(
        rename = "smuflFont",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub smufl_font: ::std::option::Option<SmuflFont>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(
        rename = "stemDirection",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stem_direction: ::std::option::Option<StemDirection>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Event> for Event {
    fn from(value: &Event) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Event {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            id: Default::default(),
            lyrics: Default::default(),
            markings: Default::default(),
            measure: Default::default(),
            notes: Default::default(),
            orient: Default::default(),
            rest: Default::default(),
            slurs: Default::default(),
            smufl_font: Default::default(),
            staff: Default::default(),
            stem_direction: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Event {
    pub fn builder() -> builder::Event {
        Default::default()
    }
}
///EventLyrics
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "lines": {
///      "type": "object",
///      "patternProperties": {
///        "^.*$": {
///          "type": "object",
///          "required": [
///            "text"
///          ],
///          "properties": {
///            "text": {
///              "$ref": "#/$defs/string"
///            },
///            "type": {
///              "type": "string",
///              "enum": [
///                "start",
///                "middle",
///                "end",
///                "whole"
///              ]
///            }
///          },
///          "additionalProperties": false
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventLyrics {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub lines: ::std::collections::HashMap<EventLyricsLinesKey, EventLyricsLinesValue>,
}
impl ::std::convert::From<&EventLyrics> for EventLyrics {
    fn from(value: &EventLyrics) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventLyrics {
    fn default() -> Self {
        Self {
            lines: Default::default(),
        }
    }
}
impl EventLyrics {
    pub fn builder() -> builder::EventLyrics {
        Default::default()
    }
}
///EventLyricsLinesKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^.*$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventLyricsLinesKey(::std::string::String);
impl ::std::ops::Deref for EventLyricsLinesKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventLyricsLinesKey> for ::std::string::String {
    fn from(value: EventLyricsLinesKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EventLyricsLinesKey> for EventLyricsLinesKey {
    fn from(value: &EventLyricsLinesKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for EventLyricsLinesKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^.*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^.*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventLyricsLinesKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventLyricsLinesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventLyricsLinesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventLyricsLinesKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///EventLyricsLinesValue
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "text"
///  ],
///  "properties": {
///    "text": {
///      "$ref": "#/$defs/string"
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "start",
///        "middle",
///        "end",
///        "whole"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventLyricsLinesValue {
    pub text: String,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<EventLyricsLinesValueType>,
}
impl ::std::convert::From<&EventLyricsLinesValue> for EventLyricsLinesValue {
    fn from(value: &EventLyricsLinesValue) -> Self {
        value.clone()
    }
}
impl EventLyricsLinesValue {
    pub fn builder() -> builder::EventLyricsLinesValue {
        Default::default()
    }
}
///EventLyricsLinesValueType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "start",
///    "middle",
///    "end",
///    "whole"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventLyricsLinesValueType {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "whole")]
    Whole,
}
impl ::std::convert::From<&Self> for EventLyricsLinesValueType {
    fn from(value: &EventLyricsLinesValueType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EventLyricsLinesValueType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Start => write!(f, "start"),
            Self::Middle => write!(f, "middle"),
            Self::End => write!(f, "end"),
            Self::Whole => write!(f, "whole"),
        }
    }
}
impl ::std::str::FromStr for EventLyricsLinesValueType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "start" => Ok(Self::Start),
            "middle" => Ok(Self::Middle),
            "end" => Ok(Self::End),
            "whole" => Ok(Self::Whole),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventLyricsLinesValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventLyricsLinesValueType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventLyricsLinesValueType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///EventMarkings
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "accent": {
///      "type": "object",
///      "properties": {
///        "pointing": {
///          "$ref": "#/$defs/up-or-down"
///        }
///      },
///      "additionalProperties": false
///    },
///    "breath": {
///      "type": "object",
///      "properties": {
///        "symbol": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "softAccent": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "spiccato": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "staccatissimo": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "staccato": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "stress": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "strongAccent": {
///      "type": "object",
///      "properties": {
///        "pointing": {
///          "$ref": "#/$defs/up-or-down"
///        }
///      },
///      "additionalProperties": false
///    },
///    "tenuto": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "tremolo": {
///      "type": "object",
///      "required": [
///        "marks"
///      ],
///      "properties": {
///        "marks": {
///          "$ref": "#/$defs/positive-integer"
///        }
///      },
///      "additionalProperties": false
///    },
///    "unstress": {
///      "type": "object",
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkings {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub accent: ::std::option::Option<EventMarkingsAccent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub breath: ::std::option::Option<EventMarkingsBreath>,
    #[serde(
        rename = "softAccent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub soft_accent: ::std::option::Option<EventMarkingsSoftAccent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub spiccato: ::std::option::Option<EventMarkingsSpiccato>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staccatissimo: ::std::option::Option<EventMarkingsStaccatissimo>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staccato: ::std::option::Option<EventMarkingsStaccato>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stress: ::std::option::Option<EventMarkingsStress>,
    #[serde(
        rename = "strongAccent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strong_accent: ::std::option::Option<EventMarkingsStrongAccent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tenuto: ::std::option::Option<EventMarkingsTenuto>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tremolo: ::std::option::Option<EventMarkingsTremolo>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub unstress: ::std::option::Option<EventMarkingsUnstress>,
}
impl ::std::convert::From<&EventMarkings> for EventMarkings {
    fn from(value: &EventMarkings) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkings {
    fn default() -> Self {
        Self {
            accent: Default::default(),
            breath: Default::default(),
            soft_accent: Default::default(),
            spiccato: Default::default(),
            staccatissimo: Default::default(),
            staccato: Default::default(),
            stress: Default::default(),
            strong_accent: Default::default(),
            tenuto: Default::default(),
            tremolo: Default::default(),
            unstress: Default::default(),
        }
    }
}
impl EventMarkings {
    pub fn builder() -> builder::EventMarkings {
        Default::default()
    }
}
///EventMarkingsAccent
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "pointing": {
///      "$ref": "#/$defs/up-or-down"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsAccent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pointing: ::std::option::Option<UpOrDown>,
}
impl ::std::convert::From<&EventMarkingsAccent> for EventMarkingsAccent {
    fn from(value: &EventMarkingsAccent) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsAccent {
    fn default() -> Self {
        Self {
            pointing: Default::default(),
        }
    }
}
impl EventMarkingsAccent {
    pub fn builder() -> builder::EventMarkingsAccent {
        Default::default()
    }
}
///EventMarkingsBreath
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "symbol": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsBreath {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub symbol: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&EventMarkingsBreath> for EventMarkingsBreath {
    fn from(value: &EventMarkingsBreath) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsBreath {
    fn default() -> Self {
        Self {
            symbol: Default::default(),
        }
    }
}
impl EventMarkingsBreath {
    pub fn builder() -> builder::EventMarkingsBreath {
        Default::default()
    }
}
///EventMarkingsSoftAccent
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsSoftAccent {}
impl ::std::convert::From<&EventMarkingsSoftAccent> for EventMarkingsSoftAccent {
    fn from(value: &EventMarkingsSoftAccent) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsSoftAccent {
    fn default() -> Self {
        Self {}
    }
}
impl EventMarkingsSoftAccent {
    pub fn builder() -> builder::EventMarkingsSoftAccent {
        Default::default()
    }
}
///EventMarkingsSpiccato
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsSpiccato {}
impl ::std::convert::From<&EventMarkingsSpiccato> for EventMarkingsSpiccato {
    fn from(value: &EventMarkingsSpiccato) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsSpiccato {
    fn default() -> Self {
        Self {}
    }
}
impl EventMarkingsSpiccato {
    pub fn builder() -> builder::EventMarkingsSpiccato {
        Default::default()
    }
}
///EventMarkingsStaccatissimo
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsStaccatissimo {}
impl ::std::convert::From<&EventMarkingsStaccatissimo> for EventMarkingsStaccatissimo {
    fn from(value: &EventMarkingsStaccatissimo) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsStaccatissimo {
    fn default() -> Self {
        Self {}
    }
}
impl EventMarkingsStaccatissimo {
    pub fn builder() -> builder::EventMarkingsStaccatissimo {
        Default::default()
    }
}
///EventMarkingsStaccato
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsStaccato {}
impl ::std::convert::From<&EventMarkingsStaccato> for EventMarkingsStaccato {
    fn from(value: &EventMarkingsStaccato) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsStaccato {
    fn default() -> Self {
        Self {}
    }
}
impl EventMarkingsStaccato {
    pub fn builder() -> builder::EventMarkingsStaccato {
        Default::default()
    }
}
///EventMarkingsStress
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsStress {}
impl ::std::convert::From<&EventMarkingsStress> for EventMarkingsStress {
    fn from(value: &EventMarkingsStress) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsStress {
    fn default() -> Self {
        Self {}
    }
}
impl EventMarkingsStress {
    pub fn builder() -> builder::EventMarkingsStress {
        Default::default()
    }
}
///EventMarkingsStrongAccent
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "pointing": {
///      "$ref": "#/$defs/up-or-down"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsStrongAccent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pointing: ::std::option::Option<UpOrDown>,
}
impl ::std::convert::From<&EventMarkingsStrongAccent> for EventMarkingsStrongAccent {
    fn from(value: &EventMarkingsStrongAccent) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsStrongAccent {
    fn default() -> Self {
        Self {
            pointing: Default::default(),
        }
    }
}
impl EventMarkingsStrongAccent {
    pub fn builder() -> builder::EventMarkingsStrongAccent {
        Default::default()
    }
}
///EventMarkingsTenuto
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsTenuto {}
impl ::std::convert::From<&EventMarkingsTenuto> for EventMarkingsTenuto {
    fn from(value: &EventMarkingsTenuto) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsTenuto {
    fn default() -> Self {
        Self {}
    }
}
impl EventMarkingsTenuto {
    pub fn builder() -> builder::EventMarkingsTenuto {
        Default::default()
    }
}
///EventMarkingsTremolo
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "marks"
///  ],
///  "properties": {
///    "marks": {
///      "$ref": "#/$defs/positive-integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsTremolo {
    pub marks: PositiveInteger,
}
impl ::std::convert::From<&EventMarkingsTremolo> for EventMarkingsTremolo {
    fn from(value: &EventMarkingsTremolo) -> Self {
        value.clone()
    }
}
impl EventMarkingsTremolo {
    pub fn builder() -> builder::EventMarkingsTremolo {
        Default::default()
    }
}
///EventMarkingsUnstress
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventMarkingsUnstress {}
impl ::std::convert::From<&EventMarkingsUnstress> for EventMarkingsUnstress {
    fn from(value: &EventMarkingsUnstress) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventMarkingsUnstress {
    fn default() -> Self {
        Self {}
    }
}
impl EventMarkingsUnstress {
    pub fn builder() -> builder::EventMarkingsUnstress {
        Default::default()
    }
}
///EventNotesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pitch"
///  ],
///  "properties": {
///    "accidentalDisplay": {
///      "type": "object",
///      "required": [
///        "show"
///      ],
///      "properties": {
///        "enclosure": {
///          "type": "object",
///          "required": [
///            "symbol"
///          ],
///          "properties": {
///            "symbol": {
///              "type": "string",
///              "enum": [
///                "parentheses",
///                "brackets"
///              ]
///            }
///          },
///          "additionalProperties": false
///        },
///        "force": {
///          "type": "boolean"
///        },
///        "show": {
///          "type": "boolean"
///        }
///      },
///      "additionalProperties": false
///    },
///    "class": {
///      "$ref": "#/$defs/style-class"
///    },
///    "id": {
///      "$ref": "#/$defs/id"
///    },
///    "perform": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "pitch": {
///      "type": "object",
///      "required": [
///        "octave",
///        "step"
///      ],
///      "properties": {
///        "alter": {
///          "type": "integer"
///        },
///        "octave": {
///          "type": "integer"
///        },
///        "step": {
///          "type": "string",
///          "enum": [
///            "A",
///            "B",
///            "C",
///            "D",
///            "E",
///            "F",
///            "G"
///          ]
///        }
///      },
///      "additionalProperties": false
///    },
///    "smuflFont": {
///      "$ref": "#/$defs/smufl-font"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    },
///    "ties": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "lv": {
///            "type": "boolean"
///          },
///          "side": {
///            "$ref": "#/$defs/slur-side"
///          },
///          "target": {
///            "$ref": "#/$defs/id"
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventNotesItem {
    #[serde(
        rename = "accidentalDisplay",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub accidental_display: ::std::option::Option<EventNotesItemAccidentalDisplay>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<StyleClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perform: ::std::option::Option<EventNotesItemPerform>,
    pub pitch: EventNotesItemPitch,
    #[serde(
        rename = "smuflFont",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub smufl_font: ::std::option::Option<SmuflFont>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub ties: ::std::vec::Vec<EventNotesItemTiesItem>,
}
impl ::std::convert::From<&EventNotesItem> for EventNotesItem {
    fn from(value: &EventNotesItem) -> Self {
        value.clone()
    }
}
impl EventNotesItem {
    pub fn builder() -> builder::EventNotesItem {
        Default::default()
    }
}
///EventNotesItemAccidentalDisplay
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "show"
///  ],
///  "properties": {
///    "enclosure": {
///      "type": "object",
///      "required": [
///        "symbol"
///      ],
///      "properties": {
///        "symbol": {
///          "type": "string",
///          "enum": [
///            "parentheses",
///            "brackets"
///          ]
///        }
///      },
///      "additionalProperties": false
///    },
///    "force": {
///      "type": "boolean"
///    },
///    "show": {
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventNotesItemAccidentalDisplay {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enclosure: ::std::option::Option<EventNotesItemAccidentalDisplayEnclosure>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: ::std::option::Option<bool>,
    pub show: bool,
}
impl ::std::convert::From<&EventNotesItemAccidentalDisplay> for EventNotesItemAccidentalDisplay {
    fn from(value: &EventNotesItemAccidentalDisplay) -> Self {
        value.clone()
    }
}
impl EventNotesItemAccidentalDisplay {
    pub fn builder() -> builder::EventNotesItemAccidentalDisplay {
        Default::default()
    }
}
///EventNotesItemAccidentalDisplayEnclosure
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "symbol"
///  ],
///  "properties": {
///    "symbol": {
///      "type": "string",
///      "enum": [
///        "parentheses",
///        "brackets"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventNotesItemAccidentalDisplayEnclosure {
    pub symbol: EventNotesItemAccidentalDisplayEnclosureSymbol,
}
impl ::std::convert::From<&EventNotesItemAccidentalDisplayEnclosure>
    for EventNotesItemAccidentalDisplayEnclosure
{
    fn from(value: &EventNotesItemAccidentalDisplayEnclosure) -> Self {
        value.clone()
    }
}
impl EventNotesItemAccidentalDisplayEnclosure {
    pub fn builder() -> builder::EventNotesItemAccidentalDisplayEnclosure {
        Default::default()
    }
}
///EventNotesItemAccidentalDisplayEnclosureSymbol
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "parentheses",
///    "brackets"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventNotesItemAccidentalDisplayEnclosureSymbol {
    #[serde(rename = "parentheses")]
    Parentheses,
    #[serde(rename = "brackets")]
    Brackets,
}
impl ::std::convert::From<&Self> for EventNotesItemAccidentalDisplayEnclosureSymbol {
    fn from(value: &EventNotesItemAccidentalDisplayEnclosureSymbol) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EventNotesItemAccidentalDisplayEnclosureSymbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Parentheses => write!(f, "parentheses"),
            Self::Brackets => write!(f, "brackets"),
        }
    }
}
impl ::std::str::FromStr for EventNotesItemAccidentalDisplayEnclosureSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "parentheses" => Ok(Self::Parentheses),
            "brackets" => Ok(Self::Brackets),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventNotesItemAccidentalDisplayEnclosureSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for EventNotesItemAccidentalDisplayEnclosureSymbol
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for EventNotesItemAccidentalDisplayEnclosureSymbol
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///EventNotesItemPerform
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventNotesItemPerform {}
impl ::std::convert::From<&EventNotesItemPerform> for EventNotesItemPerform {
    fn from(value: &EventNotesItemPerform) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventNotesItemPerform {
    fn default() -> Self {
        Self {}
    }
}
impl EventNotesItemPerform {
    pub fn builder() -> builder::EventNotesItemPerform {
        Default::default()
    }
}
///EventNotesItemPitch
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "octave",
///    "step"
///  ],
///  "properties": {
///    "alter": {
///      "type": "integer"
///    },
///    "octave": {
///      "type": "integer"
///    },
///    "step": {
///      "type": "string",
///      "enum": [
///        "A",
///        "B",
///        "C",
///        "D",
///        "E",
///        "F",
///        "G"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventNotesItemPitch {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alter: ::std::option::Option<i64>,
    pub octave: i64,
    pub step: EventNotesItemPitchStep,
}
impl ::std::convert::From<&EventNotesItemPitch> for EventNotesItemPitch {
    fn from(value: &EventNotesItemPitch) -> Self {
        value.clone()
    }
}
impl EventNotesItemPitch {
    pub fn builder() -> builder::EventNotesItemPitch {
        Default::default()
    }
}
///EventNotesItemPitchStep
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "A",
///    "B",
///    "C",
///    "D",
///    "E",
///    "F",
///    "G"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventNotesItemPitchStep {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
impl ::std::convert::From<&Self> for EventNotesItemPitchStep {
    fn from(value: &EventNotesItemPitchStep) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EventNotesItemPitchStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
        }
    }
}
impl ::std::str::FromStr for EventNotesItemPitchStep {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventNotesItemPitchStep {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventNotesItemPitchStep {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventNotesItemPitchStep {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///EventNotesItemTiesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "lv": {
///      "type": "boolean"
///    },
///    "side": {
///      "$ref": "#/$defs/slur-side"
///    },
///    "target": {
///      "$ref": "#/$defs/id"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventNotesItemTiesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lv: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub side: ::std::option::Option<SlurSide>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<Id>,
}
impl ::std::convert::From<&EventNotesItemTiesItem> for EventNotesItemTiesItem {
    fn from(value: &EventNotesItemTiesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventNotesItemTiesItem {
    fn default() -> Self {
        Self {
            lv: Default::default(),
            side: Default::default(),
            target: Default::default(),
        }
    }
}
impl EventNotesItemTiesItem {
    pub fn builder() -> builder::EventNotesItemTiesItem {
        Default::default()
    }
}
///EventRest
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "staffPosition": {
///      "$ref": "#/$defs/staff-position"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventRest {
    #[serde(
        rename = "staffPosition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub staff_position: ::std::option::Option<StaffPosition>,
}
impl ::std::convert::From<&EventRest> for EventRest {
    fn from(value: &EventRest) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventRest {
    fn default() -> Self {
        Self {
            staff_position: Default::default(),
        }
    }
}
impl EventRest {
    pub fn builder() -> builder::EventRest {
        Default::default()
    }
}
///EventSlursItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "target"
///  ],
///  "properties": {
///    "endNote": {
///      "$ref": "#/$defs/id"
///    },
///    "lineType": {
///      "type": "string",
///      "enum": [
///        "dashed",
///        "dotted",
///        "solid",
///        "wavy"
///      ]
///    },
///    "side": {
///      "$ref": "#/$defs/slur-side"
///    },
///    "sideEnd": {
///      "$ref": "#/$defs/slur-side"
///    },
///    "startNote": {
///      "$ref": "#/$defs/id"
///    },
///    "target": {
///      "$ref": "#/$defs/id"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventSlursItem {
    #[serde(
        rename = "endNote",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_note: ::std::option::Option<Id>,
    #[serde(
        rename = "lineType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub line_type: ::std::option::Option<EventSlursItemLineType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub side: ::std::option::Option<SlurSide>,
    #[serde(
        rename = "sideEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub side_end: ::std::option::Option<SlurSide>,
    #[serde(
        rename = "startNote",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_note: ::std::option::Option<Id>,
    pub target: Id,
}
impl ::std::convert::From<&EventSlursItem> for EventSlursItem {
    fn from(value: &EventSlursItem) -> Self {
        value.clone()
    }
}
impl EventSlursItem {
    pub fn builder() -> builder::EventSlursItem {
        Default::default()
    }
}
///EventSlursItemLineType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "dashed",
///    "dotted",
///    "solid",
///    "wavy"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventSlursItemLineType {
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "wavy")]
    Wavy,
}
impl ::std::convert::From<&Self> for EventSlursItemLineType {
    fn from(value: &EventSlursItemLineType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EventSlursItemLineType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dashed => write!(f, "dashed"),
            Self::Dotted => write!(f, "dotted"),
            Self::Solid => write!(f, "solid"),
            Self::Wavy => write!(f, "wavy"),
        }
    }
}
impl ::std::str::FromStr for EventSlursItemLineType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "dashed" => Ok(Self::Dashed),
            "dotted" => Ok(Self::Dotted),
            "solid" => Ok(Self::Solid),
            "wavy" => Ok(Self::Wavy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSlursItemLineType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventSlursItemLineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSlursItemLineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Fraction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "array",
///  "items": {
///    "$ref": "#/$defs/integer-unsigned"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Fraction(pub ::std::vec::Vec<IntegerUnsigned>);
impl ::std::ops::Deref for Fraction {
    type Target = ::std::vec::Vec<IntegerUnsigned>;
    fn deref(&self) -> &::std::vec::Vec<IntegerUnsigned> {
        &self.0
    }
}
impl ::std::convert::From<Fraction> for ::std::vec::Vec<IntegerUnsigned> {
    fn from(value: Fraction) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Fraction> for Fraction {
    fn from(value: &Fraction) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<IntegerUnsigned>> for Fraction {
    fn from(value: ::std::vec::Vec<IntegerUnsigned>) -> Self {
        Self(value)
    }
}
///Id
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct Id(pub ::std::string::String);
impl ::std::ops::Deref for Id {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Id> for ::std::string::String {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Id> for Id {
    fn from(value: &Id) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Id {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Id {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Id {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///IntegerUnsigned
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct IntegerUnsigned(pub i64);
impl ::std::ops::Deref for IntegerUnsigned {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<IntegerUnsigned> for i64 {
    fn from(value: IntegerUnsigned) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IntegerUnsigned> for IntegerUnsigned {
    fn from(value: &IntegerUnsigned) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for IntegerUnsigned {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for IntegerUnsigned {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for IntegerUnsigned {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for IntegerUnsigned {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for IntegerUnsigned {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for IntegerUnsigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///MeasureNumber
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MeasureNumber(pub i64);
impl ::std::ops::Deref for MeasureNumber {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<MeasureNumber> for i64 {
    fn from(value: MeasureNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MeasureNumber> for MeasureNumber {
    fn from(value: &MeasureNumber) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for MeasureNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for MeasureNumber {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for MeasureNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for MeasureNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for MeasureNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for MeasureNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///MeasureRhythmicPosition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "measure",
///    "position"
///  ],
///  "properties": {
///    "measure": {
///      "$ref": "#/$defs/measure-number"
///    },
///    "position": {
///      "$ref": "#/$defs/rhythmic-position"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MeasureRhythmicPosition {
    pub measure: MeasureNumber,
    pub position: RhythmicPosition,
}
impl ::std::convert::From<&MeasureRhythmicPosition> for MeasureRhythmicPosition {
    fn from(value: &MeasureRhythmicPosition) -> Self {
        value.clone()
    }
}
impl MeasureRhythmicPosition {
    pub fn builder() -> builder::MeasureRhythmicPosition {
        Default::default()
    }
}
///An encoding of Common Western Music Notation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "https://w3c.github.io/mnx/docs/mnx-schema.json",
///  "title": "MNX document",
///  "description": "An encoding of Common Western Music Notation.",
///  "type": "object",
///  "required": [
///    "global",
///    "mnx",
///    "parts"
///  ],
///  "properties": {
///    "global": {
///      "type": "object",
///      "required": [
///        "measures"
///      ],
///      "properties": {
///        "lyrics": {
///          "type": "object",
///          "properties": {
///            "lineMetadata": {
///              "type": "object",
///              "patternProperties": {
///                "^.*$": {
///                  "type": "object",
///                  "properties": {
///                    "label": {
///                      "type": "string"
///                    },
///                    "lang": {
///                      "type": "string"
///                    }
///                  },
///                  "additionalProperties": false
///                }
///              },
///              "additionalProperties": false
///            },
///            "lineOrder": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            }
///          },
///          "additionalProperties": false
///        },
///        "measures": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "properties": {
///              "barline": {
///                "type": "object",
///                "required": [
///                  "type"
///                ],
///                "properties": {
///                  "type": {
///                    "type": "string",
///                    "enum": [
///                      "regular",
///                      "dotted",
///                      "dashed",
///                      "heavy",
///                      "double",
///                      "final",
///                      "heavyLight",
///                      "heavyHeavy",
///                      "tick",
///                      "short",
///                      "noBarline"
///                    ]
///                  }
///                },
///                "additionalProperties": false
///              },
///              "ending": {
///                "type": "object",
///                "required": [
///                  "duration"
///                ],
///                "properties": {
///                  "class": {
///                    "$ref": "#/$defs/style-class"
///                  },
///                  "color": {
///                    "$ref": "#/$defs/color"
///                  },
///                  "duration": {
///                    "type": "integer"
///                  },
///                  "numbers": {
///                    "type": "array",
///                    "items": {
///                      "type": "integer"
///                    }
///                  },
///                  "open": {
///                    "type": "boolean"
///                  }
///                },
///                "additionalProperties": false
///              },
///              "fine": {
///                "type": "object",
///                "required": [
///                  "location"
///                ],
///                "properties": {
///                  "class": {
///                    "$ref": "#/$defs/style-class"
///                  },
///                  "color": {
///                    "$ref": "#/$defs/color"
///                  },
///                  "location": {
///                    "$ref": "#/$defs/rhythmic-position"
///                  }
///                },
///                "additionalProperties": false
///              },
///              "index": {
///                "$ref": "#/$defs/measure-number"
///              },
///              "jump": {
///                "type": "object",
///                "required": [
///                  "location",
///                  "type"
///                ],
///                "properties": {
///                  "location": {
///                    "$ref": "#/$defs/rhythmic-position"
///                  },
///                  "type": {
///                    "type": "string",
///                    "enum": [
///                      "dsalfine",
///                      "segno"
///                    ]
///                  }
///                },
///                "additionalProperties": false
///              },
///              "key": {
///                "type": "object",
///                "required": [
///                  "fifths"
///                ],
///                "properties": {
///                  "class": {
///                    "$ref": "#/$defs/style-class"
///                  },
///                  "color": {
///                    "$ref": "#/$defs/color"
///                  },
///                  "fifths": {
///                    "type": "integer"
///                  }
///                },
///                "additionalProperties": false
///              },
///              "number": {
///                "$ref": "#/$defs/measure-number"
///              },
///              "repeatEnd": {
///                "type": "object",
///                "properties": {
///                  "times": {
///                    "type": "integer"
///                  }
///                },
///                "additionalProperties": false
///              },
///              "repeatStart": {
///                "type": "object",
///                "additionalProperties": false
///              },
///              "segno": {
///                "type": "object",
///                "required": [
///                  "location"
///                ],
///                "properties": {
///                  "class": {
///                    "$ref": "#/$defs/style-class"
///                  },
///                  "color": {
///                    "$ref": "#/$defs/color"
///                  },
///                  "glyph": {
///                    "$ref": "#/$defs/smufl-glyph"
///                  },
///                  "location": {
///                    "$ref": "#/$defs/rhythmic-position"
///                  }
///                },
///                "additionalProperties": false
///              },
///              "tempos": {
///                "type": "array",
///                "items": {
///                  "type": "object",
///                  "required": [
///                    "bpm",
///                    "value"
///                  ],
///                  "properties": {
///                    "bpm": {
///                      "type": "integer"
///                    },
///                    "location": {
///                      "$ref": "#/$defs/rhythmic-position"
///                    },
///                    "value": {
///                      "$ref": "#/$defs/note-value"
///                    }
///                  },
///                  "additionalProperties": false
///                }
///              },
///              "time": {
///                "type": "object",
///                "required": [
///                  "count",
///                  "unit"
///                ],
///                "properties": {
///                  "count": {
///                    "$ref": "#/$defs/positive-integer"
///                  },
///                  "unit": {
///                    "type": "integer",
///                    "enum": [
///                      1,
///                      2,
///                      4,
///                      8,
///                      16,
///                      32,
///                      64,
///                      128
///                    ]
///                  }
///                },
///                "additionalProperties": false
///              }
///            },
///            "additionalProperties": false
///          }
///        },
///        "styles": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "selector"
///            ],
///            "properties": {
///              "color": {
///                "$ref": "#/$defs/color"
///              },
///              "selector": {
///                "type": "string"
///              }
///            },
///            "additionalProperties": false
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    "layouts": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "content",
///          "id"
///        ],
///        "properties": {
///          "content": {
///            "$ref": "#/$defs/system-layout-content"
///          },
///          "id": {
///            "$ref": "#/$defs/id"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "mnx": {
///      "type": "object",
///      "required": [
///        "version"
///      ],
///      "properties": {
///        "support": {
///          "type": "object",
///          "properties": {
///            "useAccidentalDisplay": {
///              "type": "boolean"
///            }
///          },
///          "additionalProperties": false
///        },
///        "version": {
///          "type": "integer"
///        }
///      },
///      "additionalProperties": false
///    },
///    "parts": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "id": {
///            "$ref": "#/$defs/id"
///          },
///          "measures": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "sequences"
///              ],
///              "properties": {
///                "beams": {
///                  "$ref": "#/$defs/beam-list"
///                },
///                "clefs": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "clef"
///                    ],
///                    "properties": {
///                      "clef": {
///                        "type": "object",
///                        "required": [
///                          "sign",
///                          "staffPosition"
///                        ],
///                        "properties": {
///                          "class": {
///                            "$ref": "#/$defs/style-class"
///                          },
///                          "color": {
///                            "type": "string"
///                          },
///                          "glyph": {
///                            "$ref": "#/$defs/smufl-glyph"
///                          },
///                          "octave": {
///                            "type": "integer"
///                          },
///                          "sign": {
///                            "type": "string",
///                            "enum": [
///                              "C",
///                              "F",
///                              "G"
///                            ]
///                          },
///                          "staffPosition": {
///                            "$ref": "#/$defs/staff-position"
///                          }
///                        },
///                        "additionalProperties": false
///                      },
///                      "position": {
///                        "$ref": "#/$defs/rhythmic-position"
///                      },
///                      "staff": {
///                        "$ref": "#/$defs/staff-number"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                },
///                "dynamics": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "position",
///                      "value"
///                    ],
///                    "properties": {
///                      "glyph": {
///                        "$ref": "#/$defs/smufl-glyph"
///                      },
///                      "position": {
///                        "$ref": "#/$defs/rhythmic-position"
///                      },
///                      "staff": {
///                        "$ref": "#/$defs/staff-number"
///                      },
///                      "value": {
///                        "type": "string"
///                      },
///                      "voice": {
///                        "$ref": "#/$defs/voice-name"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                },
///                "ottavas": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "end",
///                      "position",
///                      "value"
///                    ],
///                    "properties": {
///                      "end": {
///                        "$ref": "#/$defs/measure-rhythmic-position"
///                      },
///                      "orient": {
///                        "$ref": "#/$defs/orientation"
///                      },
///                      "position": {
///                        "$ref": "#/$defs/rhythmic-position"
///                      },
///                      "staff": {
///                        "$ref": "#/$defs/staff-number"
///                      },
///                      "value": {
///                        "type": "integer",
///                        "enum": [
///                          1,
///                          2,
///                          -1,
///                          -2,
///                          3,
///                          -3
///                        ]
///                      },
///                      "voice": {
///                        "$ref": "#/$defs/voice-name"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                },
///                "sequences": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "content"
///                    ],
///                    "properties": {
///                      "content": {
///                        "$ref": "#/$defs/sequence-content"
///                      },
///                      "orient": {
///                        "$ref": "#/$defs/orientation"
///                      },
///                      "staff": {
///                        "$ref": "#/$defs/staff-number"
///                      },
///                      "voice": {
///                        "$ref": "#/$defs/voice-name"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "name": {
///            "type": "string"
///          },
///          "shortName": {
///            "type": "string"
///          },
///          "smuflFont": {
///            "$ref": "#/$defs/smufl-font"
///          },
///          "staves": {
///            "type": "integer"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "scores": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "layout": {
///            "$ref": "#/$defs/id"
///          },
///          "multimeasureRests": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "duration",
///                "start"
///              ],
///              "properties": {
///                "duration": {
///                  "type": "integer"
///                },
///                "label": {
///                  "$ref": "#/$defs/string"
///                },
///                "start": {
///                  "$ref": "#/$defs/measure-number"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "name": {
///            "type": "string"
///          },
///          "pages": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "systems"
///              ],
///              "properties": {
///                "layout": {
///                  "$ref": "#/$defs/id"
///                },
///                "systems": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "measure"
///                    ],
///                    "properties": {
///                      "layout": {
///                        "$ref": "#/$defs/id"
///                      },
///                      "layoutChanges": {
///                        "type": "array",
///                        "items": {
///                          "type": "object",
///                          "required": [
///                            "layout",
///                            "location"
///                          ],
///                          "properties": {
///                            "layout": {
///                              "$ref": "#/$defs/id"
///                            },
///                            "location": {
///                              "$ref": "#/$defs/measure-rhythmic-position"
///                            }
///                          },
///                          "additionalProperties": false
///                        }
///                      },
///                      "measure": {
///                        "$ref": "#/$defs/measure-number"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                }
///              },
///              "additionalProperties": false
///            }
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocument {
    pub global: MnxDocumentGlobal,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub layouts: ::std::vec::Vec<MnxDocumentLayoutsItem>,
    pub mnx: MnxDocumentMnx,
    pub parts: ::std::vec::Vec<MnxDocumentPartsItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scores: ::std::vec::Vec<MnxDocumentScoresItem>,
}
impl ::std::convert::From<&MnxDocument> for MnxDocument {
    fn from(value: &MnxDocument) -> Self {
        value.clone()
    }
}
impl MnxDocument {
    pub fn builder() -> builder::MnxDocument {
        Default::default()
    }
}
///MnxDocumentGlobal
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "measures"
///  ],
///  "properties": {
///    "lyrics": {
///      "type": "object",
///      "properties": {
///        "lineMetadata": {
///          "type": "object",
///          "patternProperties": {
///            "^.*$": {
///              "type": "object",
///              "properties": {
///                "label": {
///                  "type": "string"
///                },
///                "lang": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "additionalProperties": false
///        },
///        "lineOrder": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    "measures": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "barline": {
///            "type": "object",
///            "required": [
///              "type"
///            ],
///            "properties": {
///              "type": {
///                "type": "string",
///                "enum": [
///                  "regular",
///                  "dotted",
///                  "dashed",
///                  "heavy",
///                  "double",
///                  "final",
///                  "heavyLight",
///                  "heavyHeavy",
///                  "tick",
///                  "short",
///                  "noBarline"
///                ]
///              }
///            },
///            "additionalProperties": false
///          },
///          "ending": {
///            "type": "object",
///            "required": [
///              "duration"
///            ],
///            "properties": {
///              "class": {
///                "$ref": "#/$defs/style-class"
///              },
///              "color": {
///                "$ref": "#/$defs/color"
///              },
///              "duration": {
///                "type": "integer"
///              },
///              "numbers": {
///                "type": "array",
///                "items": {
///                  "type": "integer"
///                }
///              },
///              "open": {
///                "type": "boolean"
///              }
///            },
///            "additionalProperties": false
///          },
///          "fine": {
///            "type": "object",
///            "required": [
///              "location"
///            ],
///            "properties": {
///              "class": {
///                "$ref": "#/$defs/style-class"
///              },
///              "color": {
///                "$ref": "#/$defs/color"
///              },
///              "location": {
///                "$ref": "#/$defs/rhythmic-position"
///              }
///            },
///            "additionalProperties": false
///          },
///          "index": {
///            "$ref": "#/$defs/measure-number"
///          },
///          "jump": {
///            "type": "object",
///            "required": [
///              "location",
///              "type"
///            ],
///            "properties": {
///              "location": {
///                "$ref": "#/$defs/rhythmic-position"
///              },
///              "type": {
///                "type": "string",
///                "enum": [
///                  "dsalfine",
///                  "segno"
///                ]
///              }
///            },
///            "additionalProperties": false
///          },
///          "key": {
///            "type": "object",
///            "required": [
///              "fifths"
///            ],
///            "properties": {
///              "class": {
///                "$ref": "#/$defs/style-class"
///              },
///              "color": {
///                "$ref": "#/$defs/color"
///              },
///              "fifths": {
///                "type": "integer"
///              }
///            },
///            "additionalProperties": false
///          },
///          "number": {
///            "$ref": "#/$defs/measure-number"
///          },
///          "repeatEnd": {
///            "type": "object",
///            "properties": {
///              "times": {
///                "type": "integer"
///              }
///            },
///            "additionalProperties": false
///          },
///          "repeatStart": {
///            "type": "object",
///            "additionalProperties": false
///          },
///          "segno": {
///            "type": "object",
///            "required": [
///              "location"
///            ],
///            "properties": {
///              "class": {
///                "$ref": "#/$defs/style-class"
///              },
///              "color": {
///                "$ref": "#/$defs/color"
///              },
///              "glyph": {
///                "$ref": "#/$defs/smufl-glyph"
///              },
///              "location": {
///                "$ref": "#/$defs/rhythmic-position"
///              }
///            },
///            "additionalProperties": false
///          },
///          "tempos": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "bpm",
///                "value"
///              ],
///              "properties": {
///                "bpm": {
///                  "type": "integer"
///                },
///                "location": {
///                  "$ref": "#/$defs/rhythmic-position"
///                },
///                "value": {
///                  "$ref": "#/$defs/note-value"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "time": {
///            "type": "object",
///            "required": [
///              "count",
///              "unit"
///            ],
///            "properties": {
///              "count": {
///                "$ref": "#/$defs/positive-integer"
///              },
///              "unit": {
///                "type": "integer",
///                "enum": [
///                  1,
///                  2,
///                  4,
///                  8,
///                  16,
///                  32,
///                  64,
///                  128
///                ]
///              }
///            },
///            "additionalProperties": false
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "styles": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "selector"
///        ],
///        "properties": {
///          "color": {
///            "$ref": "#/$defs/color"
///          },
///          "selector": {
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobal {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lyrics: ::std::option::Option<MnxDocumentGlobalLyrics>,
    pub measures: ::std::vec::Vec<MnxDocumentGlobalMeasuresItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub styles: ::std::vec::Vec<MnxDocumentGlobalStylesItem>,
}
impl ::std::convert::From<&MnxDocumentGlobal> for MnxDocumentGlobal {
    fn from(value: &MnxDocumentGlobal) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobal {
    pub fn builder() -> builder::MnxDocumentGlobal {
        Default::default()
    }
}
///MnxDocumentGlobalLyrics
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "lineMetadata": {
///      "type": "object",
///      "patternProperties": {
///        "^.*$": {
///          "type": "object",
///          "properties": {
///            "label": {
///              "type": "string"
///            },
///            "lang": {
///              "type": "string"
///            }
///          },
///          "additionalProperties": false
///        }
///      },
///      "additionalProperties": false
///    },
///    "lineOrder": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalLyrics {
    #[serde(
        rename = "lineMetadata",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub line_metadata: ::std::collections::HashMap<
        MnxDocumentGlobalLyricsLineMetadataKey,
        MnxDocumentGlobalLyricsLineMetadataValue,
    >,
    #[serde(
        rename = "lineOrder",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub line_order: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&MnxDocumentGlobalLyrics> for MnxDocumentGlobalLyrics {
    fn from(value: &MnxDocumentGlobalLyrics) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MnxDocumentGlobalLyrics {
    fn default() -> Self {
        Self {
            line_metadata: Default::default(),
            line_order: Default::default(),
        }
    }
}
impl MnxDocumentGlobalLyrics {
    pub fn builder() -> builder::MnxDocumentGlobalLyrics {
        Default::default()
    }
}
///MnxDocumentGlobalLyricsLineMetadataKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^.*$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MnxDocumentGlobalLyricsLineMetadataKey(::std::string::String);
impl ::std::ops::Deref for MnxDocumentGlobalLyricsLineMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MnxDocumentGlobalLyricsLineMetadataKey> for ::std::string::String {
    fn from(value: MnxDocumentGlobalLyricsLineMetadataKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MnxDocumentGlobalLyricsLineMetadataKey>
    for MnxDocumentGlobalLyricsLineMetadataKey
{
    fn from(value: &MnxDocumentGlobalLyricsLineMetadataKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for MnxDocumentGlobalLyricsLineMetadataKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^.*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^.*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MnxDocumentGlobalLyricsLineMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MnxDocumentGlobalLyricsLineMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MnxDocumentGlobalLyricsLineMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MnxDocumentGlobalLyricsLineMetadataKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///MnxDocumentGlobalLyricsLineMetadataValue
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "label": {
///      "type": "string"
///    },
///    "lang": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalLyricsLineMetadataValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lang: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&MnxDocumentGlobalLyricsLineMetadataValue>
    for MnxDocumentGlobalLyricsLineMetadataValue
{
    fn from(value: &MnxDocumentGlobalLyricsLineMetadataValue) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MnxDocumentGlobalLyricsLineMetadataValue {
    fn default() -> Self {
        Self {
            label: Default::default(),
            lang: Default::default(),
        }
    }
}
impl MnxDocumentGlobalLyricsLineMetadataValue {
    pub fn builder() -> builder::MnxDocumentGlobalLyricsLineMetadataValue {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "barline": {
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "regular",
///            "dotted",
///            "dashed",
///            "heavy",
///            "double",
///            "final",
///            "heavyLight",
///            "heavyHeavy",
///            "tick",
///            "short",
///            "noBarline"
///          ]
///        }
///      },
///      "additionalProperties": false
///    },
///    "ending": {
///      "type": "object",
///      "required": [
///        "duration"
///      ],
///      "properties": {
///        "class": {
///          "$ref": "#/$defs/style-class"
///        },
///        "color": {
///          "$ref": "#/$defs/color"
///        },
///        "duration": {
///          "type": "integer"
///        },
///        "numbers": {
///          "type": "array",
///          "items": {
///            "type": "integer"
///          }
///        },
///        "open": {
///          "type": "boolean"
///        }
///      },
///      "additionalProperties": false
///    },
///    "fine": {
///      "type": "object",
///      "required": [
///        "location"
///      ],
///      "properties": {
///        "class": {
///          "$ref": "#/$defs/style-class"
///        },
///        "color": {
///          "$ref": "#/$defs/color"
///        },
///        "location": {
///          "$ref": "#/$defs/rhythmic-position"
///        }
///      },
///      "additionalProperties": false
///    },
///    "index": {
///      "$ref": "#/$defs/measure-number"
///    },
///    "jump": {
///      "type": "object",
///      "required": [
///        "location",
///        "type"
///      ],
///      "properties": {
///        "location": {
///          "$ref": "#/$defs/rhythmic-position"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "dsalfine",
///            "segno"
///          ]
///        }
///      },
///      "additionalProperties": false
///    },
///    "key": {
///      "type": "object",
///      "required": [
///        "fifths"
///      ],
///      "properties": {
///        "class": {
///          "$ref": "#/$defs/style-class"
///        },
///        "color": {
///          "$ref": "#/$defs/color"
///        },
///        "fifths": {
///          "type": "integer"
///        }
///      },
///      "additionalProperties": false
///    },
///    "number": {
///      "$ref": "#/$defs/measure-number"
///    },
///    "repeatEnd": {
///      "type": "object",
///      "properties": {
///        "times": {
///          "type": "integer"
///        }
///      },
///      "additionalProperties": false
///    },
///    "repeatStart": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "segno": {
///      "type": "object",
///      "required": [
///        "location"
///      ],
///      "properties": {
///        "class": {
///          "$ref": "#/$defs/style-class"
///        },
///        "color": {
///          "$ref": "#/$defs/color"
///        },
///        "glyph": {
///          "$ref": "#/$defs/smufl-glyph"
///        },
///        "location": {
///          "$ref": "#/$defs/rhythmic-position"
///        }
///      },
///      "additionalProperties": false
///    },
///    "tempos": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "bpm",
///          "value"
///        ],
///        "properties": {
///          "bpm": {
///            "type": "integer"
///          },
///          "location": {
///            "$ref": "#/$defs/rhythmic-position"
///          },
///          "value": {
///            "$ref": "#/$defs/note-value"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "time": {
///      "type": "object",
///      "required": [
///        "count",
///        "unit"
///      ],
///      "properties": {
///        "count": {
///          "$ref": "#/$defs/positive-integer"
///        },
///        "unit": {
///          "type": "integer",
///          "enum": [
///            1,
///            2,
///            4,
///            8,
///            16,
///            32,
///            64,
///            128
///          ]
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub barline: ::std::option::Option<MnxDocumentGlobalMeasuresItemBarline>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ending: ::std::option::Option<MnxDocumentGlobalMeasuresItemEnding>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fine: ::std::option::Option<MnxDocumentGlobalMeasuresItemFine>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<MeasureNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub jump: ::std::option::Option<MnxDocumentGlobalMeasuresItemJump>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub key: ::std::option::Option<MnxDocumentGlobalMeasuresItemKey>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub number: ::std::option::Option<MeasureNumber>,
    #[serde(
        rename = "repeatEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub repeat_end: ::std::option::Option<MnxDocumentGlobalMeasuresItemRepeatEnd>,
    #[serde(
        rename = "repeatStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub repeat_start: ::std::option::Option<MnxDocumentGlobalMeasuresItemRepeatStart>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub segno: ::std::option::Option<MnxDocumentGlobalMeasuresItemSegno>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tempos: ::std::vec::Vec<MnxDocumentGlobalMeasuresItemTemposItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time: ::std::option::Option<MnxDocumentGlobalMeasuresItemTime>,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItem> for MnxDocumentGlobalMeasuresItem {
    fn from(value: &MnxDocumentGlobalMeasuresItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MnxDocumentGlobalMeasuresItem {
    fn default() -> Self {
        Self {
            barline: Default::default(),
            ending: Default::default(),
            fine: Default::default(),
            index: Default::default(),
            jump: Default::default(),
            key: Default::default(),
            number: Default::default(),
            repeat_end: Default::default(),
            repeat_start: Default::default(),
            segno: Default::default(),
            tempos: Default::default(),
            time: Default::default(),
        }
    }
}
impl MnxDocumentGlobalMeasuresItem {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItem {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemBarline
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "type": {
///      "type": "string",
///      "enum": [
///        "regular",
///        "dotted",
///        "dashed",
///        "heavy",
///        "double",
///        "final",
///        "heavyLight",
///        "heavyHeavy",
///        "tick",
///        "short",
///        "noBarline"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemBarline {
    #[serde(rename = "type")]
    pub type_: MnxDocumentGlobalMeasuresItemBarlineType,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemBarline>
    for MnxDocumentGlobalMeasuresItemBarline
{
    fn from(value: &MnxDocumentGlobalMeasuresItemBarline) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemBarline {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemBarline {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemBarlineType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "regular",
///    "dotted",
///    "dashed",
///    "heavy",
///    "double",
///    "final",
///    "heavyLight",
///    "heavyHeavy",
///    "tick",
///    "short",
///    "noBarline"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MnxDocumentGlobalMeasuresItemBarlineType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "dotted")]
    Dotted,
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "heavy")]
    Heavy,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "final")]
    Final,
    #[serde(rename = "heavyLight")]
    HeavyLight,
    #[serde(rename = "heavyHeavy")]
    HeavyHeavy,
    #[serde(rename = "tick")]
    Tick,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "noBarline")]
    NoBarline,
}
impl ::std::convert::From<&Self> for MnxDocumentGlobalMeasuresItemBarlineType {
    fn from(value: &MnxDocumentGlobalMeasuresItemBarlineType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MnxDocumentGlobalMeasuresItemBarlineType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Regular => write!(f, "regular"),
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
            Self::Heavy => write!(f, "heavy"),
            Self::Double => write!(f, "double"),
            Self::Final => write!(f, "final"),
            Self::HeavyLight => write!(f, "heavyLight"),
            Self::HeavyHeavy => write!(f, "heavyHeavy"),
            Self::Tick => write!(f, "tick"),
            Self::Short => write!(f, "short"),
            Self::NoBarline => write!(f, "noBarline"),
        }
    }
}
impl ::std::str::FromStr for MnxDocumentGlobalMeasuresItemBarlineType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "regular" => Ok(Self::Regular),
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "heavy" => Ok(Self::Heavy),
            "double" => Ok(Self::Double),
            "final" => Ok(Self::Final),
            "heavyLight" => Ok(Self::HeavyLight),
            "heavyHeavy" => Ok(Self::HeavyHeavy),
            "tick" => Ok(Self::Tick),
            "short" => Ok(Self::Short),
            "noBarline" => Ok(Self::NoBarline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MnxDocumentGlobalMeasuresItemBarlineType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MnxDocumentGlobalMeasuresItemBarlineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MnxDocumentGlobalMeasuresItemBarlineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///MnxDocumentGlobalMeasuresItemEnding
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "duration"
///  ],
///  "properties": {
///    "class": {
///      "$ref": "#/$defs/style-class"
///    },
///    "color": {
///      "$ref": "#/$defs/color"
///    },
///    "duration": {
///      "type": "integer"
///    },
///    "numbers": {
///      "type": "array",
///      "items": {
///        "type": "integer"
///      }
///    },
///    "open": {
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemEnding {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<StyleClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub duration: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub numbers: ::std::vec::Vec<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub open: ::std::option::Option<bool>,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemEnding>
    for MnxDocumentGlobalMeasuresItemEnding
{
    fn from(value: &MnxDocumentGlobalMeasuresItemEnding) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemEnding {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemEnding {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemFine
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "location"
///  ],
///  "properties": {
///    "class": {
///      "$ref": "#/$defs/style-class"
///    },
///    "color": {
///      "$ref": "#/$defs/color"
///    },
///    "location": {
///      "$ref": "#/$defs/rhythmic-position"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemFine {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<StyleClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub location: RhythmicPosition,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemFine>
    for MnxDocumentGlobalMeasuresItemFine
{
    fn from(value: &MnxDocumentGlobalMeasuresItemFine) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemFine {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemFine {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemJump
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "location",
///    "type"
///  ],
///  "properties": {
///    "location": {
///      "$ref": "#/$defs/rhythmic-position"
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "dsalfine",
///        "segno"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemJump {
    pub location: RhythmicPosition,
    #[serde(rename = "type")]
    pub type_: MnxDocumentGlobalMeasuresItemJumpType,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemJump>
    for MnxDocumentGlobalMeasuresItemJump
{
    fn from(value: &MnxDocumentGlobalMeasuresItemJump) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemJump {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemJump {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemJumpType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "dsalfine",
///    "segno"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MnxDocumentGlobalMeasuresItemJumpType {
    #[serde(rename = "dsalfine")]
    Dsalfine,
    #[serde(rename = "segno")]
    Segno,
}
impl ::std::convert::From<&Self> for MnxDocumentGlobalMeasuresItemJumpType {
    fn from(value: &MnxDocumentGlobalMeasuresItemJumpType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MnxDocumentGlobalMeasuresItemJumpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dsalfine => write!(f, "dsalfine"),
            Self::Segno => write!(f, "segno"),
        }
    }
}
impl ::std::str::FromStr for MnxDocumentGlobalMeasuresItemJumpType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "dsalfine" => Ok(Self::Dsalfine),
            "segno" => Ok(Self::Segno),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MnxDocumentGlobalMeasuresItemJumpType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MnxDocumentGlobalMeasuresItemJumpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MnxDocumentGlobalMeasuresItemJumpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///MnxDocumentGlobalMeasuresItemKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "fifths"
///  ],
///  "properties": {
///    "class": {
///      "$ref": "#/$defs/style-class"
///    },
///    "color": {
///      "$ref": "#/$defs/color"
///    },
///    "fifths": {
///      "type": "integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemKey {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<StyleClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub fifths: i64,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemKey> for MnxDocumentGlobalMeasuresItemKey {
    fn from(value: &MnxDocumentGlobalMeasuresItemKey) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemKey {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemKey {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemRepeatEnd
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "times": {
///      "type": "integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemRepeatEnd {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times: ::std::option::Option<i64>,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemRepeatEnd>
    for MnxDocumentGlobalMeasuresItemRepeatEnd
{
    fn from(value: &MnxDocumentGlobalMeasuresItemRepeatEnd) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MnxDocumentGlobalMeasuresItemRepeatEnd {
    fn default() -> Self {
        Self {
            times: Default::default(),
        }
    }
}
impl MnxDocumentGlobalMeasuresItemRepeatEnd {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemRepeatEnd {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemRepeatStart
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemRepeatStart {}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemRepeatStart>
    for MnxDocumentGlobalMeasuresItemRepeatStart
{
    fn from(value: &MnxDocumentGlobalMeasuresItemRepeatStart) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MnxDocumentGlobalMeasuresItemRepeatStart {
    fn default() -> Self {
        Self {}
    }
}
impl MnxDocumentGlobalMeasuresItemRepeatStart {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemRepeatStart {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemSegno
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "location"
///  ],
///  "properties": {
///    "class": {
///      "$ref": "#/$defs/style-class"
///    },
///    "color": {
///      "$ref": "#/$defs/color"
///    },
///    "glyph": {
///      "$ref": "#/$defs/smufl-glyph"
///    },
///    "location": {
///      "$ref": "#/$defs/rhythmic-position"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemSegno {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<StyleClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub glyph: ::std::option::Option<SmuflGlyph>,
    pub location: RhythmicPosition,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemSegno>
    for MnxDocumentGlobalMeasuresItemSegno
{
    fn from(value: &MnxDocumentGlobalMeasuresItemSegno) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemSegno {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemSegno {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemTemposItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "bpm",
///    "value"
///  ],
///  "properties": {
///    "bpm": {
///      "type": "integer"
///    },
///    "location": {
///      "$ref": "#/$defs/rhythmic-position"
///    },
///    "value": {
///      "$ref": "#/$defs/note-value"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemTemposItem {
    pub bpm: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<RhythmicPosition>,
    pub value: NoteValue,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemTemposItem>
    for MnxDocumentGlobalMeasuresItemTemposItem
{
    fn from(value: &MnxDocumentGlobalMeasuresItemTemposItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemTemposItem {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemTemposItem {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemTime
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "count",
///    "unit"
///  ],
///  "properties": {
///    "count": {
///      "$ref": "#/$defs/positive-integer"
///    },
///    "unit": {
///      "type": "integer",
///      "enum": [
///        1,
///        2,
///        4,
///        8,
///        16,
///        32,
///        64,
///        128
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalMeasuresItemTime {
    pub count: PositiveInteger,
    pub unit: MnxDocumentGlobalMeasuresItemTimeUnit,
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemTime>
    for MnxDocumentGlobalMeasuresItemTime
{
    fn from(value: &MnxDocumentGlobalMeasuresItemTime) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalMeasuresItemTime {
    pub fn builder() -> builder::MnxDocumentGlobalMeasuresItemTime {
        Default::default()
    }
}
///MnxDocumentGlobalMeasuresItemTimeUnit
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    1,
///    2,
///    4,
///    8,
///    16,
///    32,
///    64,
///    128
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MnxDocumentGlobalMeasuresItemTimeUnit(i64);
impl ::std::ops::Deref for MnxDocumentGlobalMeasuresItemTimeUnit {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<MnxDocumentGlobalMeasuresItemTimeUnit> for i64 {
    fn from(value: MnxDocumentGlobalMeasuresItemTimeUnit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MnxDocumentGlobalMeasuresItemTimeUnit>
    for MnxDocumentGlobalMeasuresItemTimeUnit
{
    fn from(value: &MnxDocumentGlobalMeasuresItemTimeUnit) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for MnxDocumentGlobalMeasuresItemTimeUnit {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64, 4_i64, 8_i64, 16_i64, 32_i64, 64_i64, 128_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for MnxDocumentGlobalMeasuresItemTimeUnit {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
///MnxDocumentGlobalStylesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "selector"
///  ],
///  "properties": {
///    "color": {
///      "$ref": "#/$defs/color"
///    },
///    "selector": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentGlobalStylesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub selector: ::std::string::String,
}
impl ::std::convert::From<&MnxDocumentGlobalStylesItem> for MnxDocumentGlobalStylesItem {
    fn from(value: &MnxDocumentGlobalStylesItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentGlobalStylesItem {
    pub fn builder() -> builder::MnxDocumentGlobalStylesItem {
        Default::default()
    }
}
///MnxDocumentLayoutsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "content",
///    "id"
///  ],
///  "properties": {
///    "content": {
///      "$ref": "#/$defs/system-layout-content"
///    },
///    "id": {
///      "$ref": "#/$defs/id"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentLayoutsItem {
    pub content: SystemLayoutContent,
    pub id: Id,
}
impl ::std::convert::From<&MnxDocumentLayoutsItem> for MnxDocumentLayoutsItem {
    fn from(value: &MnxDocumentLayoutsItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentLayoutsItem {
    pub fn builder() -> builder::MnxDocumentLayoutsItem {
        Default::default()
    }
}
///MnxDocumentMnx
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "version"
///  ],
///  "properties": {
///    "support": {
///      "type": "object",
///      "properties": {
///        "useAccidentalDisplay": {
///          "type": "boolean"
///        }
///      },
///      "additionalProperties": false
///    },
///    "version": {
///      "type": "integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentMnx {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub support: ::std::option::Option<MnxDocumentMnxSupport>,
    pub version: i64,
}
impl ::std::convert::From<&MnxDocumentMnx> for MnxDocumentMnx {
    fn from(value: &MnxDocumentMnx) -> Self {
        value.clone()
    }
}
impl MnxDocumentMnx {
    pub fn builder() -> builder::MnxDocumentMnx {
        Default::default()
    }
}
///MnxDocumentMnxSupport
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "useAccidentalDisplay": {
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentMnxSupport {
    #[serde(
        rename = "useAccidentalDisplay",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_accidental_display: ::std::option::Option<bool>,
}
impl ::std::convert::From<&MnxDocumentMnxSupport> for MnxDocumentMnxSupport {
    fn from(value: &MnxDocumentMnxSupport) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MnxDocumentMnxSupport {
    fn default() -> Self {
        Self {
            use_accidental_display: Default::default(),
        }
    }
}
impl MnxDocumentMnxSupport {
    pub fn builder() -> builder::MnxDocumentMnxSupport {
        Default::default()
    }
}
///MnxDocumentPartsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/id"
///    },
///    "measures": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "sequences"
///        ],
///        "properties": {
///          "beams": {
///            "$ref": "#/$defs/beam-list"
///          },
///          "clefs": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "clef"
///              ],
///              "properties": {
///                "clef": {
///                  "type": "object",
///                  "required": [
///                    "sign",
///                    "staffPosition"
///                  ],
///                  "properties": {
///                    "class": {
///                      "$ref": "#/$defs/style-class"
///                    },
///                    "color": {
///                      "type": "string"
///                    },
///                    "glyph": {
///                      "$ref": "#/$defs/smufl-glyph"
///                    },
///                    "octave": {
///                      "type": "integer"
///                    },
///                    "sign": {
///                      "type": "string",
///                      "enum": [
///                        "C",
///                        "F",
///                        "G"
///                      ]
///                    },
///                    "staffPosition": {
///                      "$ref": "#/$defs/staff-position"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                "position": {
///                  "$ref": "#/$defs/rhythmic-position"
///                },
///                "staff": {
///                  "$ref": "#/$defs/staff-number"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "dynamics": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "position",
///                "value"
///              ],
///              "properties": {
///                "glyph": {
///                  "$ref": "#/$defs/smufl-glyph"
///                },
///                "position": {
///                  "$ref": "#/$defs/rhythmic-position"
///                },
///                "staff": {
///                  "$ref": "#/$defs/staff-number"
///                },
///                "value": {
///                  "type": "string"
///                },
///                "voice": {
///                  "$ref": "#/$defs/voice-name"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "ottavas": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "end",
///                "position",
///                "value"
///              ],
///              "properties": {
///                "end": {
///                  "$ref": "#/$defs/measure-rhythmic-position"
///                },
///                "orient": {
///                  "$ref": "#/$defs/orientation"
///                },
///                "position": {
///                  "$ref": "#/$defs/rhythmic-position"
///                },
///                "staff": {
///                  "$ref": "#/$defs/staff-number"
///                },
///                "value": {
///                  "type": "integer",
///                  "enum": [
///                    1,
///                    2,
///                    -1,
///                    -2,
///                    3,
///                    -3
///                  ]
///                },
///                "voice": {
///                  "$ref": "#/$defs/voice-name"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "sequences": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "content"
///              ],
///              "properties": {
///                "content": {
///                  "$ref": "#/$defs/sequence-content"
///                },
///                "orient": {
///                  "$ref": "#/$defs/orientation"
///                },
///                "staff": {
///                  "$ref": "#/$defs/staff-number"
///                },
///                "voice": {
///                  "$ref": "#/$defs/voice-name"
///                }
///              },
///              "additionalProperties": false
///            }
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "name": {
///      "type": "string"
///    },
///    "shortName": {
///      "type": "string"
///    },
///    "smuflFont": {
///      "$ref": "#/$defs/smufl-font"
///    },
///    "staves": {
///      "type": "integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentPartsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub measures: ::std::vec::Vec<MnxDocumentPartsItemMeasuresItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "shortName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "smuflFont",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub smufl_font: ::std::option::Option<SmuflFont>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staves: ::std::option::Option<i64>,
}
impl ::std::convert::From<&MnxDocumentPartsItem> for MnxDocumentPartsItem {
    fn from(value: &MnxDocumentPartsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MnxDocumentPartsItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            measures: Default::default(),
            name: Default::default(),
            short_name: Default::default(),
            smufl_font: Default::default(),
            staves: Default::default(),
        }
    }
}
impl MnxDocumentPartsItem {
    pub fn builder() -> builder::MnxDocumentPartsItem {
        Default::default()
    }
}
///MnxDocumentPartsItemMeasuresItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "sequences"
///  ],
///  "properties": {
///    "beams": {
///      "$ref": "#/$defs/beam-list"
///    },
///    "clefs": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "clef"
///        ],
///        "properties": {
///          "clef": {
///            "type": "object",
///            "required": [
///              "sign",
///              "staffPosition"
///            ],
///            "properties": {
///              "class": {
///                "$ref": "#/$defs/style-class"
///              },
///              "color": {
///                "type": "string"
///              },
///              "glyph": {
///                "$ref": "#/$defs/smufl-glyph"
///              },
///              "octave": {
///                "type": "integer"
///              },
///              "sign": {
///                "type": "string",
///                "enum": [
///                  "C",
///                  "F",
///                  "G"
///                ]
///              },
///              "staffPosition": {
///                "$ref": "#/$defs/staff-position"
///              }
///            },
///            "additionalProperties": false
///          },
///          "position": {
///            "$ref": "#/$defs/rhythmic-position"
///          },
///          "staff": {
///            "$ref": "#/$defs/staff-number"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "dynamics": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "position",
///          "value"
///        ],
///        "properties": {
///          "glyph": {
///            "$ref": "#/$defs/smufl-glyph"
///          },
///          "position": {
///            "$ref": "#/$defs/rhythmic-position"
///          },
///          "staff": {
///            "$ref": "#/$defs/staff-number"
///          },
///          "value": {
///            "type": "string"
///          },
///          "voice": {
///            "$ref": "#/$defs/voice-name"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "ottavas": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "end",
///          "position",
///          "value"
///        ],
///        "properties": {
///          "end": {
///            "$ref": "#/$defs/measure-rhythmic-position"
///          },
///          "orient": {
///            "$ref": "#/$defs/orientation"
///          },
///          "position": {
///            "$ref": "#/$defs/rhythmic-position"
///          },
///          "staff": {
///            "$ref": "#/$defs/staff-number"
///          },
///          "value": {
///            "type": "integer",
///            "enum": [
///              1,
///              2,
///              -1,
///              -2,
///              3,
///              -3
///            ]
///          },
///          "voice": {
///            "$ref": "#/$defs/voice-name"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "sequences": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "content"
///        ],
///        "properties": {
///          "content": {
///            "$ref": "#/$defs/sequence-content"
///          },
///          "orient": {
///            "$ref": "#/$defs/orientation"
///          },
///          "staff": {
///            "$ref": "#/$defs/staff-number"
///          },
///          "voice": {
///            "$ref": "#/$defs/voice-name"
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentPartsItemMeasuresItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub beams: ::std::option::Option<BeamList>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub clefs: ::std::vec::Vec<MnxDocumentPartsItemMeasuresItemClefsItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub dynamics: ::std::vec::Vec<MnxDocumentPartsItemMeasuresItemDynamicsItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub ottavas: ::std::vec::Vec<MnxDocumentPartsItemMeasuresItemOttavasItem>,
    pub sequences: ::std::vec::Vec<MnxDocumentPartsItemMeasuresItemSequencesItem>,
}
impl ::std::convert::From<&MnxDocumentPartsItemMeasuresItem> for MnxDocumentPartsItemMeasuresItem {
    fn from(value: &MnxDocumentPartsItemMeasuresItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentPartsItemMeasuresItem {
    pub fn builder() -> builder::MnxDocumentPartsItemMeasuresItem {
        Default::default()
    }
}
///MnxDocumentPartsItemMeasuresItemClefsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "clef"
///  ],
///  "properties": {
///    "clef": {
///      "type": "object",
///      "required": [
///        "sign",
///        "staffPosition"
///      ],
///      "properties": {
///        "class": {
///          "$ref": "#/$defs/style-class"
///        },
///        "color": {
///          "type": "string"
///        },
///        "glyph": {
///          "$ref": "#/$defs/smufl-glyph"
///        },
///        "octave": {
///          "type": "integer"
///        },
///        "sign": {
///          "type": "string",
///          "enum": [
///            "C",
///            "F",
///            "G"
///          ]
///        },
///        "staffPosition": {
///          "$ref": "#/$defs/staff-position"
///        }
///      },
///      "additionalProperties": false
///    },
///    "position": {
///      "$ref": "#/$defs/rhythmic-position"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentPartsItemMeasuresItemClefsItem {
    pub clef: MnxDocumentPartsItemMeasuresItemClefsItemClef,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub position: ::std::option::Option<RhythmicPosition>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
}
impl ::std::convert::From<&MnxDocumentPartsItemMeasuresItemClefsItem>
    for MnxDocumentPartsItemMeasuresItemClefsItem
{
    fn from(value: &MnxDocumentPartsItemMeasuresItemClefsItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentPartsItemMeasuresItemClefsItem {
    pub fn builder() -> builder::MnxDocumentPartsItemMeasuresItemClefsItem {
        Default::default()
    }
}
///MnxDocumentPartsItemMeasuresItemClefsItemClef
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "sign",
///    "staffPosition"
///  ],
///  "properties": {
///    "class": {
///      "$ref": "#/$defs/style-class"
///    },
///    "color": {
///      "type": "string"
///    },
///    "glyph": {
///      "$ref": "#/$defs/smufl-glyph"
///    },
///    "octave": {
///      "type": "integer"
///    },
///    "sign": {
///      "type": "string",
///      "enum": [
///        "C",
///        "F",
///        "G"
///      ]
///    },
///    "staffPosition": {
///      "$ref": "#/$defs/staff-position"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentPartsItemMeasuresItemClefsItemClef {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<StyleClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub glyph: ::std::option::Option<SmuflGlyph>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub octave: ::std::option::Option<i64>,
    pub sign: MnxDocumentPartsItemMeasuresItemClefsItemClefSign,
    #[serde(rename = "staffPosition")]
    pub staff_position: StaffPosition,
}
impl ::std::convert::From<&MnxDocumentPartsItemMeasuresItemClefsItemClef>
    for MnxDocumentPartsItemMeasuresItemClefsItemClef
{
    fn from(value: &MnxDocumentPartsItemMeasuresItemClefsItemClef) -> Self {
        value.clone()
    }
}
impl MnxDocumentPartsItemMeasuresItemClefsItemClef {
    pub fn builder() -> builder::MnxDocumentPartsItemMeasuresItemClefsItemClef {
        Default::default()
    }
}
///MnxDocumentPartsItemMeasuresItemClefsItemClefSign
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "C",
///    "F",
///    "G"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MnxDocumentPartsItemMeasuresItemClefsItemClefSign {
    C,
    F,
    G,
}
impl ::std::convert::From<&Self> for MnxDocumentPartsItemMeasuresItemClefsItemClefSign {
    fn from(value: &MnxDocumentPartsItemMeasuresItemClefsItemClefSign) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MnxDocumentPartsItemMeasuresItemClefsItemClefSign {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::C => write!(f, "C"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
        }
    }
}
impl ::std::str::FromStr for MnxDocumentPartsItemMeasuresItemClefsItemClefSign {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "C" => Ok(Self::C),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MnxDocumentPartsItemMeasuresItemClefsItemClefSign {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for MnxDocumentPartsItemMeasuresItemClefsItemClefSign
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MnxDocumentPartsItemMeasuresItemClefsItemClefSign
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///MnxDocumentPartsItemMeasuresItemDynamicsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "position",
///    "value"
///  ],
///  "properties": {
///    "glyph": {
///      "$ref": "#/$defs/smufl-glyph"
///    },
///    "position": {
///      "$ref": "#/$defs/rhythmic-position"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    },
///    "value": {
///      "type": "string"
///    },
///    "voice": {
///      "$ref": "#/$defs/voice-name"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentPartsItemMeasuresItemDynamicsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub glyph: ::std::option::Option<SmuflGlyph>,
    pub position: RhythmicPosition,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    pub value: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
}
impl ::std::convert::From<&MnxDocumentPartsItemMeasuresItemDynamicsItem>
    for MnxDocumentPartsItemMeasuresItemDynamicsItem
{
    fn from(value: &MnxDocumentPartsItemMeasuresItemDynamicsItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentPartsItemMeasuresItemDynamicsItem {
    pub fn builder() -> builder::MnxDocumentPartsItemMeasuresItemDynamicsItem {
        Default::default()
    }
}
///MnxDocumentPartsItemMeasuresItemOttavasItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end",
///    "position",
///    "value"
///  ],
///  "properties": {
///    "end": {
///      "$ref": "#/$defs/measure-rhythmic-position"
///    },
///    "orient": {
///      "$ref": "#/$defs/orientation"
///    },
///    "position": {
///      "$ref": "#/$defs/rhythmic-position"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    },
///    "value": {
///      "type": "integer",
///      "enum": [
///        1,
///        2,
///        -1,
///        -2,
///        3,
///        -3
///      ]
///    },
///    "voice": {
///      "$ref": "#/$defs/voice-name"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentPartsItemMeasuresItemOttavasItem {
    pub end: MeasureRhythmicPosition,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orient: ::std::option::Option<Orientation>,
    pub position: RhythmicPosition,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    pub value: MnxDocumentPartsItemMeasuresItemOttavasItemValue,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
}
impl ::std::convert::From<&MnxDocumentPartsItemMeasuresItemOttavasItem>
    for MnxDocumentPartsItemMeasuresItemOttavasItem
{
    fn from(value: &MnxDocumentPartsItemMeasuresItemOttavasItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentPartsItemMeasuresItemOttavasItem {
    pub fn builder() -> builder::MnxDocumentPartsItemMeasuresItemOttavasItem {
        Default::default()
    }
}
///MnxDocumentPartsItemMeasuresItemOttavasItemValue
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    1,
///    2,
///    -1,
///    -2,
///    3,
///    -3
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MnxDocumentPartsItemMeasuresItemOttavasItemValue(i64);
impl ::std::ops::Deref for MnxDocumentPartsItemMeasuresItemOttavasItemValue {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<MnxDocumentPartsItemMeasuresItemOttavasItemValue> for i64 {
    fn from(value: MnxDocumentPartsItemMeasuresItemOttavasItemValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MnxDocumentPartsItemMeasuresItemOttavasItemValue>
    for MnxDocumentPartsItemMeasuresItemOttavasItemValue
{
    fn from(value: &MnxDocumentPartsItemMeasuresItemOttavasItemValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for MnxDocumentPartsItemMeasuresItemOttavasItemValue {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64, -1_i64, -2_i64, 3_i64, -3_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for MnxDocumentPartsItemMeasuresItemOttavasItemValue {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
///MnxDocumentPartsItemMeasuresItemSequencesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "content"
///  ],
///  "properties": {
///    "content": {
///      "$ref": "#/$defs/sequence-content"
///    },
///    "orient": {
///      "$ref": "#/$defs/orientation"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    },
///    "voice": {
///      "$ref": "#/$defs/voice-name"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentPartsItemMeasuresItemSequencesItem {
    pub content: SequenceContent,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orient: ::std::option::Option<Orientation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
}
impl ::std::convert::From<&MnxDocumentPartsItemMeasuresItemSequencesItem>
    for MnxDocumentPartsItemMeasuresItemSequencesItem
{
    fn from(value: &MnxDocumentPartsItemMeasuresItemSequencesItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentPartsItemMeasuresItemSequencesItem {
    pub fn builder() -> builder::MnxDocumentPartsItemMeasuresItemSequencesItem {
        Default::default()
    }
}
///MnxDocumentScoresItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "layout": {
///      "$ref": "#/$defs/id"
///    },
///    "multimeasureRests": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "duration",
///          "start"
///        ],
///        "properties": {
///          "duration": {
///            "type": "integer"
///          },
///          "label": {
///            "$ref": "#/$defs/string"
///          },
///          "start": {
///            "$ref": "#/$defs/measure-number"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "name": {
///      "type": "string"
///    },
///    "pages": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "systems"
///        ],
///        "properties": {
///          "layout": {
///            "$ref": "#/$defs/id"
///          },
///          "systems": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "measure"
///              ],
///              "properties": {
///                "layout": {
///                  "$ref": "#/$defs/id"
///                },
///                "layoutChanges": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "layout",
///                      "location"
///                    ],
///                    "properties": {
///                      "layout": {
///                        "$ref": "#/$defs/id"
///                      },
///                      "location": {
///                        "$ref": "#/$defs/measure-rhythmic-position"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                },
///                "measure": {
///                  "$ref": "#/$defs/measure-number"
///                }
///              },
///              "additionalProperties": false
///            }
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentScoresItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<Id>,
    #[serde(
        rename = "multimeasureRests",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub multimeasure_rests: ::std::vec::Vec<MnxDocumentScoresItemMultimeasureRestsItem>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub pages: ::std::vec::Vec<MnxDocumentScoresItemPagesItem>,
}
impl ::std::convert::From<&MnxDocumentScoresItem> for MnxDocumentScoresItem {
    fn from(value: &MnxDocumentScoresItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentScoresItem {
    pub fn builder() -> builder::MnxDocumentScoresItem {
        Default::default()
    }
}
///MnxDocumentScoresItemMultimeasureRestsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "duration",
///    "start"
///  ],
///  "properties": {
///    "duration": {
///      "type": "integer"
///    },
///    "label": {
///      "$ref": "#/$defs/string"
///    },
///    "start": {
///      "$ref": "#/$defs/measure-number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentScoresItemMultimeasureRestsItem {
    pub duration: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<String>,
    pub start: MeasureNumber,
}
impl ::std::convert::From<&MnxDocumentScoresItemMultimeasureRestsItem>
    for MnxDocumentScoresItemMultimeasureRestsItem
{
    fn from(value: &MnxDocumentScoresItemMultimeasureRestsItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentScoresItemMultimeasureRestsItem {
    pub fn builder() -> builder::MnxDocumentScoresItemMultimeasureRestsItem {
        Default::default()
    }
}
///MnxDocumentScoresItemPagesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "systems"
///  ],
///  "properties": {
///    "layout": {
///      "$ref": "#/$defs/id"
///    },
///    "systems": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "measure"
///        ],
///        "properties": {
///          "layout": {
///            "$ref": "#/$defs/id"
///          },
///          "layoutChanges": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "layout",
///                "location"
///              ],
///              "properties": {
///                "layout": {
///                  "$ref": "#/$defs/id"
///                },
///                "location": {
///                  "$ref": "#/$defs/measure-rhythmic-position"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "measure": {
///            "$ref": "#/$defs/measure-number"
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentScoresItemPagesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<Id>,
    pub systems: ::std::vec::Vec<MnxDocumentScoresItemPagesItemSystemsItem>,
}
impl ::std::convert::From<&MnxDocumentScoresItemPagesItem> for MnxDocumentScoresItemPagesItem {
    fn from(value: &MnxDocumentScoresItemPagesItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentScoresItemPagesItem {
    pub fn builder() -> builder::MnxDocumentScoresItemPagesItem {
        Default::default()
    }
}
///MnxDocumentScoresItemPagesItemSystemsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "measure"
///  ],
///  "properties": {
///    "layout": {
///      "$ref": "#/$defs/id"
///    },
///    "layoutChanges": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "layout",
///          "location"
///        ],
///        "properties": {
///          "layout": {
///            "$ref": "#/$defs/id"
///          },
///          "location": {
///            "$ref": "#/$defs/measure-rhythmic-position"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "measure": {
///      "$ref": "#/$defs/measure-number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentScoresItemPagesItemSystemsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<Id>,
    #[serde(
        rename = "layoutChanges",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub layout_changes: ::std::vec::Vec<MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem>,
    pub measure: MeasureNumber,
}
impl ::std::convert::From<&MnxDocumentScoresItemPagesItemSystemsItem>
    for MnxDocumentScoresItemPagesItemSystemsItem
{
    fn from(value: &MnxDocumentScoresItemPagesItemSystemsItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentScoresItemPagesItemSystemsItem {
    pub fn builder() -> builder::MnxDocumentScoresItemPagesItemSystemsItem {
        Default::default()
    }
}
///MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "layout",
///    "location"
///  ],
///  "properties": {
///    "layout": {
///      "$ref": "#/$defs/id"
///    },
///    "location": {
///      "$ref": "#/$defs/measure-rhythmic-position"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem {
    pub layout: Id,
    pub location: MeasureRhythmicPosition,
}
impl ::std::convert::From<&MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem>
    for MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem
{
    fn from(value: &MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem) -> Self {
        value.clone()
    }
}
impl MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem {
    pub fn builder() -> builder::MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem {
        Default::default()
    }
}
///NoteValue
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "base"
///  ],
///  "properties": {
///    "base": {
///      "type": "string",
///      "enum": [
///        "duplexMaxima",
///        "maxima",
///        "longa",
///        "breve",
///        "whole",
///        "half",
///        "quarter",
///        "eighth",
///        "16th",
///        "32nd",
///        "64th",
///        "128th",
///        "256th",
///        "512th",
///        "1024th",
///        "2048th",
///        "4096th"
///      ]
///    },
///    "dots": {
///      "$ref": "#/$defs/positive-integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NoteValue {
    pub base: NoteValueBase,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dots: ::std::option::Option<PositiveInteger>,
}
impl ::std::convert::From<&NoteValue> for NoteValue {
    fn from(value: &NoteValue) -> Self {
        value.clone()
    }
}
impl NoteValue {
    pub fn builder() -> builder::NoteValue {
        Default::default()
    }
}
///NoteValueBase
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "duplexMaxima",
///    "maxima",
///    "longa",
///    "breve",
///    "whole",
///    "half",
///    "quarter",
///    "eighth",
///    "16th",
///    "32nd",
///    "64th",
///    "128th",
///    "256th",
///    "512th",
///    "1024th",
///    "2048th",
///    "4096th"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NoteValueBase {
    #[serde(rename = "duplexMaxima")]
    DuplexMaxima,
    #[serde(rename = "maxima")]
    Maxima,
    #[serde(rename = "longa")]
    Longa,
    #[serde(rename = "breve")]
    Breve,
    #[serde(rename = "whole")]
    Whole,
    #[serde(rename = "half")]
    Half,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "eighth")]
    Eighth,
    #[serde(rename = "16th")]
    _16th,
    #[serde(rename = "32nd")]
    _32nd,
    #[serde(rename = "64th")]
    _64th,
    #[serde(rename = "128th")]
    _128th,
    #[serde(rename = "256th")]
    _256th,
    #[serde(rename = "512th")]
    _512th,
    #[serde(rename = "1024th")]
    _1024th,
    #[serde(rename = "2048th")]
    _2048th,
    #[serde(rename = "4096th")]
    _4096th,
}
impl ::std::convert::From<&Self> for NoteValueBase {
    fn from(value: &NoteValueBase) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for NoteValueBase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DuplexMaxima => write!(f, "duplexMaxima"),
            Self::Maxima => write!(f, "maxima"),
            Self::Longa => write!(f, "longa"),
            Self::Breve => write!(f, "breve"),
            Self::Whole => write!(f, "whole"),
            Self::Half => write!(f, "half"),
            Self::Quarter => write!(f, "quarter"),
            Self::Eighth => write!(f, "eighth"),
            Self::_16th => write!(f, "16th"),
            Self::_32nd => write!(f, "32nd"),
            Self::_64th => write!(f, "64th"),
            Self::_128th => write!(f, "128th"),
            Self::_256th => write!(f, "256th"),
            Self::_512th => write!(f, "512th"),
            Self::_1024th => write!(f, "1024th"),
            Self::_2048th => write!(f, "2048th"),
            Self::_4096th => write!(f, "4096th"),
        }
    }
}
impl ::std::str::FromStr for NoteValueBase {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "duplexMaxima" => Ok(Self::DuplexMaxima),
            "maxima" => Ok(Self::Maxima),
            "longa" => Ok(Self::Longa),
            "breve" => Ok(Self::Breve),
            "whole" => Ok(Self::Whole),
            "half" => Ok(Self::Half),
            "quarter" => Ok(Self::Quarter),
            "eighth" => Ok(Self::Eighth),
            "16th" => Ok(Self::_16th),
            "32nd" => Ok(Self::_32nd),
            "64th" => Ok(Self::_64th),
            "128th" => Ok(Self::_128th),
            "256th" => Ok(Self::_256th),
            "512th" => Ok(Self::_512th),
            "1024th" => Ok(Self::_1024th),
            "2048th" => Ok(Self::_2048th),
            "4096th" => Ok(Self::_4096th),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NoteValueBase {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NoteValueBase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NoteValueBase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///NoteValueQuantity
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "duration",
///    "multiple"
///  ],
///  "properties": {
///    "duration": {
///      "$ref": "#/$defs/note-value"
///    },
///    "multiple": {
///      "$ref": "#/$defs/positive-integer"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NoteValueQuantity {
    pub duration: NoteValue,
    pub multiple: PositiveInteger,
}
impl ::std::convert::From<&NoteValueQuantity> for NoteValueQuantity {
    fn from(value: &NoteValueQuantity) -> Self {
        value.clone()
    }
}
impl NoteValueQuantity {
    pub fn builder() -> builder::NoteValueQuantity {
        Default::default()
    }
}
///Orientation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct Orientation(pub ::std::string::String);
impl ::std::ops::Deref for Orientation {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Orientation> for ::std::string::String {
    fn from(value: Orientation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Orientation> for Orientation {
    fn from(value: &Orientation) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Orientation {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Orientation {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///PositiveInteger
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PositiveInteger(pub i64);
impl ::std::ops::Deref for PositiveInteger {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<PositiveInteger> for i64 {
    fn from(value: PositiveInteger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PositiveInteger> for PositiveInteger {
    fn from(value: &PositiveInteger) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for PositiveInteger {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PositiveInteger {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PositiveInteger {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for PositiveInteger {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PositiveInteger {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for PositiveInteger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///RhythmicPosition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "fraction"
///  ],
///  "properties": {
///    "fraction": {
///      "$ref": "#/$defs/fraction"
///    },
///    "graceIndex": {
///      "$ref": "#/$defs/integer-unsigned"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RhythmicPosition {
    pub fraction: Fraction,
    #[serde(
        rename = "graceIndex",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grace_index: ::std::option::Option<IntegerUnsigned>,
}
impl ::std::convert::From<&RhythmicPosition> for RhythmicPosition {
    fn from(value: &RhythmicPosition) -> Self {
        value.clone()
    }
}
impl RhythmicPosition {
    pub fn builder() -> builder::RhythmicPosition {
        Default::default()
    }
}
///SequenceContent
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "array",
///  "items": {
///    "anyOf": [
///      {
///        "$ref": "#/$defs/event"
///      },
///      {
///        "type": "object",
///        "required": [
///          "content",
///          "type"
///        ],
///        "properties": {
///          "class": {
///            "$ref": "#/$defs/style-class"
///          },
///          "color": {
///            "$ref": "#/$defs/color"
///          },
///          "content": {
///            "type": "array",
///            "items": {
///              "$ref": "#/$defs/event"
///            }
///          },
///          "graceType": {
///            "type": "string",
///            "enum": [
///              "makeTime",
///              "stealFollowing",
///              "stealPrevious"
///            ]
///          },
///          "slash": {
///            "type": "boolean"
///          },
///          "type": {
///            "type": "string",
///            "const": "grace"
///          }
///        },
///        "additionalProperties": false
///      },
///      {
///        "type": "object",
///        "required": [
///          "content",
///          "inner",
///          "outer",
///          "type"
///        ],
///        "properties": {
///          "bracket": {
///            "type": "string",
///            "enum": [
///              "yes",
///              "no",
///              "auto"
///            ]
///          },
///          "content": {
///            "$ref": "#/$defs/sequence-content"
///          },
///          "inner": {
///            "$ref": "#/$defs/note-value-quantity"
///          },
///          "orient": {
///            "$ref": "#/$defs/orientation"
///          },
///          "outer": {
///            "$ref": "#/$defs/note-value-quantity"
///          },
///          "showNumber": {
///            "$ref": "#/$defs/tuplet-display-setting"
///          },
///          "showValue": {
///            "$ref": "#/$defs/tuplet-display-setting"
///          },
///          "staff": {
///            "$ref": "#/$defs/staff-number"
///          },
///          "type": {
///            "type": "string",
///            "const": "tuplet"
///          }
///        },
///        "additionalProperties": false
///      },
///      {
///        "type": "object",
///        "required": [
///          "duration",
///          "type"
///        ],
///        "properties": {
///          "duration": {
///            "$ref": "#/$defs/fraction"
///          },
///          "type": {
///            "type": "string",
///            "const": "space"
///          }
///        },
///        "additionalProperties": false
///      }
///    ]
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SequenceContent(pub ::std::vec::Vec<SequenceContentItem>);
impl ::std::ops::Deref for SequenceContent {
    type Target = ::std::vec::Vec<SequenceContentItem>;
    fn deref(&self) -> &::std::vec::Vec<SequenceContentItem> {
        &self.0
    }
}
impl ::std::convert::From<SequenceContent> for ::std::vec::Vec<SequenceContentItem> {
    fn from(value: SequenceContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SequenceContent> for SequenceContent {
    fn from(value: &SequenceContent) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<SequenceContentItem>> for SequenceContent {
    fn from(value: ::std::vec::Vec<SequenceContentItem>) -> Self {
        Self(value)
    }
}
///SequenceContentItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "$ref": "#/$defs/event"
///    },
///    {
///      "type": "object",
///      "required": [
///        "content",
///        "type"
///      ],
///      "properties": {
///        "class": {
///          "$ref": "#/$defs/style-class"
///        },
///        "color": {
///          "$ref": "#/$defs/color"
///        },
///        "content": {
///          "type": "array",
///          "items": {
///            "$ref": "#/$defs/event"
///          }
///        },
///        "graceType": {
///          "type": "string",
///          "enum": [
///            "makeTime",
///            "stealFollowing",
///            "stealPrevious"
///          ]
///        },
///        "slash": {
///          "type": "boolean"
///        },
///        "type": {
///          "type": "string",
///          "const": "grace"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "content",
///        "inner",
///        "outer",
///        "type"
///      ],
///      "properties": {
///        "bracket": {
///          "type": "string",
///          "enum": [
///            "yes",
///            "no",
///            "auto"
///          ]
///        },
///        "content": {
///          "$ref": "#/$defs/sequence-content"
///        },
///        "inner": {
///          "$ref": "#/$defs/note-value-quantity"
///        },
///        "orient": {
///          "$ref": "#/$defs/orientation"
///        },
///        "outer": {
///          "$ref": "#/$defs/note-value-quantity"
///        },
///        "showNumber": {
///          "$ref": "#/$defs/tuplet-display-setting"
///        },
///        "showValue": {
///          "$ref": "#/$defs/tuplet-display-setting"
///        },
///        "staff": {
///          "$ref": "#/$defs/staff-number"
///        },
///        "type": {
///          "type": "string",
///          "const": "tuplet"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "duration",
///        "type"
///      ],
///      "properties": {
///        "duration": {
///          "$ref": "#/$defs/fraction"
///        },
///        "type": {
///          "type": "string",
///          "const": "space"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SequenceContentItem {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<Event>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<SequenceContentItemSubtype1>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<SequenceContentItemSubtype2>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_3: ::std::option::Option<SequenceContentItemSubtype3>,
}
impl ::std::convert::From<&SequenceContentItem> for SequenceContentItem {
    fn from(value: &SequenceContentItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SequenceContentItem {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
            subtype_3: Default::default(),
        }
    }
}
impl SequenceContentItem {
    pub fn builder() -> builder::SequenceContentItem {
        Default::default()
    }
}
///SequenceContentItemSubtype1
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "content",
///    "type"
///  ],
///  "properties": {
///    "class": {
///      "$ref": "#/$defs/style-class"
///    },
///    "color": {
///      "$ref": "#/$defs/color"
///    },
///    "content": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/event"
///      }
///    },
///    "graceType": {
///      "type": "string",
///      "enum": [
///        "makeTime",
///        "stealFollowing",
///        "stealPrevious"
///      ]
///    },
///    "slash": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string",
///      "const": "grace"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SequenceContentItemSubtype1 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<StyleClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub content: ::std::vec::Vec<Event>,
    #[serde(
        rename = "graceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grace_type: ::std::option::Option<SequenceContentItemSubtype1GraceType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub slash: ::std::option::Option<bool>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&SequenceContentItemSubtype1> for SequenceContentItemSubtype1 {
    fn from(value: &SequenceContentItemSubtype1) -> Self {
        value.clone()
    }
}
impl SequenceContentItemSubtype1 {
    pub fn builder() -> builder::SequenceContentItemSubtype1 {
        Default::default()
    }
}
///SequenceContentItemSubtype1GraceType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "makeTime",
///    "stealFollowing",
///    "stealPrevious"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SequenceContentItemSubtype1GraceType {
    #[serde(rename = "makeTime")]
    MakeTime,
    #[serde(rename = "stealFollowing")]
    StealFollowing,
    #[serde(rename = "stealPrevious")]
    StealPrevious,
}
impl ::std::convert::From<&Self> for SequenceContentItemSubtype1GraceType {
    fn from(value: &SequenceContentItemSubtype1GraceType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SequenceContentItemSubtype1GraceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MakeTime => write!(f, "makeTime"),
            Self::StealFollowing => write!(f, "stealFollowing"),
            Self::StealPrevious => write!(f, "stealPrevious"),
        }
    }
}
impl ::std::str::FromStr for SequenceContentItemSubtype1GraceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "makeTime" => Ok(Self::MakeTime),
            "stealFollowing" => Ok(Self::StealFollowing),
            "stealPrevious" => Ok(Self::StealPrevious),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SequenceContentItemSubtype1GraceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SequenceContentItemSubtype1GraceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SequenceContentItemSubtype1GraceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///SequenceContentItemSubtype2
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "content",
///    "inner",
///    "outer",
///    "type"
///  ],
///  "properties": {
///    "bracket": {
///      "type": "string",
///      "enum": [
///        "yes",
///        "no",
///        "auto"
///      ]
///    },
///    "content": {
///      "$ref": "#/$defs/sequence-content"
///    },
///    "inner": {
///      "$ref": "#/$defs/note-value-quantity"
///    },
///    "orient": {
///      "$ref": "#/$defs/orientation"
///    },
///    "outer": {
///      "$ref": "#/$defs/note-value-quantity"
///    },
///    "showNumber": {
///      "$ref": "#/$defs/tuplet-display-setting"
///    },
///    "showValue": {
///      "$ref": "#/$defs/tuplet-display-setting"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    },
///    "type": {
///      "type": "string",
///      "const": "tuplet"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SequenceContentItemSubtype2 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bracket: ::std::option::Option<SequenceContentItemSubtype2Bracket>,
    pub content: SequenceContent,
    pub inner: NoteValueQuantity,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orient: ::std::option::Option<Orientation>,
    pub outer: NoteValueQuantity,
    #[serde(
        rename = "showNumber",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_number: ::std::option::Option<TupletDisplaySetting>,
    #[serde(
        rename = "showValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_value: ::std::option::Option<TupletDisplaySetting>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&SequenceContentItemSubtype2> for SequenceContentItemSubtype2 {
    fn from(value: &SequenceContentItemSubtype2) -> Self {
        value.clone()
    }
}
impl SequenceContentItemSubtype2 {
    pub fn builder() -> builder::SequenceContentItemSubtype2 {
        Default::default()
    }
}
///SequenceContentItemSubtype2Bracket
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "yes",
///    "no",
///    "auto"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SequenceContentItemSubtype2Bracket {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::convert::From<&Self> for SequenceContentItemSubtype2Bracket {
    fn from(value: &SequenceContentItemSubtype2Bracket) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SequenceContentItemSubtype2Bracket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Yes => write!(f, "yes"),
            Self::No => write!(f, "no"),
            Self::Auto => write!(f, "auto"),
        }
    }
}
impl ::std::str::FromStr for SequenceContentItemSubtype2Bracket {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "yes" => Ok(Self::Yes),
            "no" => Ok(Self::No),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SequenceContentItemSubtype2Bracket {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SequenceContentItemSubtype2Bracket {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SequenceContentItemSubtype2Bracket {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///SequenceContentItemSubtype3
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "duration",
///    "type"
///  ],
///  "properties": {
///    "duration": {
///      "$ref": "#/$defs/fraction"
///    },
///    "type": {
///      "type": "string",
///      "const": "space"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SequenceContentItemSubtype3 {
    pub duration: Fraction,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&SequenceContentItemSubtype3> for SequenceContentItemSubtype3 {
    fn from(value: &SequenceContentItemSubtype3) -> Self {
        value.clone()
    }
}
impl SequenceContentItemSubtype3 {
    pub fn builder() -> builder::SequenceContentItemSubtype3 {
        Default::default()
    }
}
///SlurSide
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "up",
///    "down"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SlurSide {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}
impl ::std::convert::From<&Self> for SlurSide {
    fn from(value: &SlurSide) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SlurSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => write!(f, "up"),
            Self::Down => write!(f, "down"),
        }
    }
}
impl ::std::str::FromStr for SlurSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SlurSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SlurSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SlurSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///SmuflFont
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct SmuflFont(pub ::std::string::String);
impl ::std::ops::Deref for SmuflFont {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SmuflFont> for ::std::string::String {
    fn from(value: SmuflFont) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmuflFont> for SmuflFont {
    fn from(value: &SmuflFont) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for SmuflFont {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SmuflFont {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SmuflFont {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///SmuflGlyph
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct SmuflGlyph(pub ::std::string::String);
impl ::std::ops::Deref for SmuflGlyph {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SmuflGlyph> for ::std::string::String {
    fn from(value: SmuflGlyph) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmuflGlyph> for SmuflGlyph {
    fn from(value: &SmuflGlyph) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for SmuflGlyph {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SmuflGlyph {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SmuflGlyph {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///StaffLabel
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct StaffLabel(pub ::std::string::String);
impl ::std::ops::Deref for StaffLabel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StaffLabel> for ::std::string::String {
    fn from(value: StaffLabel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StaffLabel> for StaffLabel {
    fn from(value: &StaffLabel) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for StaffLabel {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffLabel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StaffLabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///StaffLabelref
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct StaffLabelref(pub ::std::string::String);
impl ::std::ops::Deref for StaffLabelref {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StaffLabelref> for ::std::string::String {
    fn from(value: StaffLabelref) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StaffLabelref> for StaffLabelref {
    fn from(value: &StaffLabelref) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for StaffLabelref {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffLabelref {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StaffLabelref {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///StaffNumber
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StaffNumber(pub i64);
impl ::std::ops::Deref for StaffNumber {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<StaffNumber> for i64 {
    fn from(value: StaffNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StaffNumber> for StaffNumber {
    fn from(value: &StaffNumber) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for StaffNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffNumber {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for StaffNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for StaffNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for StaffNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for StaffNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///StaffPosition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StaffPosition(pub i64);
impl ::std::ops::Deref for StaffPosition {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<StaffPosition> for i64 {
    fn from(value: StaffPosition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StaffPosition> for StaffPosition {
    fn from(value: &StaffPosition) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for StaffPosition {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffPosition {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for StaffPosition {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for StaffPosition {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for StaffPosition {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for StaffPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///StaffSymbol
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "bracket",
///    "brace",
///    "noSymbol"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StaffSymbol {
    #[serde(rename = "bracket")]
    Bracket,
    #[serde(rename = "brace")]
    Brace,
    #[serde(rename = "noSymbol")]
    NoSymbol,
}
impl ::std::convert::From<&Self> for StaffSymbol {
    fn from(value: &StaffSymbol) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StaffSymbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bracket => write!(f, "bracket"),
            Self::Brace => write!(f, "brace"),
            Self::NoSymbol => write!(f, "noSymbol"),
        }
    }
}
impl ::std::str::FromStr for StaffSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bracket" => Ok(Self::Bracket),
            "brace" => Ok(Self::Brace),
            "noSymbol" => Ok(Self::NoSymbol),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StaffSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StaffSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StaffSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///StemDirection
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "up",
///    "down"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StemDirection {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}
impl ::std::convert::From<&Self> for StemDirection {
    fn from(value: &StemDirection) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StemDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => write!(f, "up"),
            Self::Down => write!(f, "down"),
        }
    }
}
impl ::std::str::FromStr for StemDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StemDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StemDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StemDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///String
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct String(pub ::std::string::String);
impl ::std::ops::Deref for String {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<String> for ::std::string::String {
    fn from(value: String) -> Self {
        value.0
    }
}
impl ::std::convert::From<&String> for String {
    fn from(value: &String) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for String {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for String {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for String {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///StyleClass
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct StyleClass(pub ::std::string::String);
impl ::std::ops::Deref for StyleClass {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StyleClass> for ::std::string::String {
    fn from(value: StyleClass) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StyleClass> for StyleClass {
    fn from(value: &StyleClass) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for StyleClass {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StyleClass {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StyleClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///SystemLayoutContent
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "array",
///  "items": {
///    "anyOf": [
///      {
///        "type": "object",
///        "required": [
///          "content",
///          "type"
///        ],
///        "properties": {
///          "content": {
///            "$ref": "#/$defs/system-layout-content"
///          },
///          "label": {
///            "$ref": "#/$defs/staff-label"
///          },
///          "symbol": {
///            "$ref": "#/$defs/staff-symbol"
///          },
///          "type": {
///            "type": "string",
///            "const": "group"
///          }
///        },
///        "additionalProperties": false
///      },
///      {
///        "type": "object",
///        "required": [
///          "sources",
///          "type"
///        ],
///        "properties": {
///          "label": {
///            "$ref": "#/$defs/staff-label"
///          },
///          "labelref": {
///            "$ref": "#/$defs/staff-labelref"
///          },
///          "sources": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "part"
///              ],
///              "properties": {
///                "label": {
///                  "$ref": "#/$defs/staff-label"
///                },
///                "labelref": {
///                  "$ref": "#/$defs/staff-labelref"
///                },
///                "part": {
///                  "$ref": "#/$defs/id"
///                },
///                "staff": {
///                  "$ref": "#/$defs/staff-number"
///                },
///                "stem": {
///                  "$ref": "#/$defs/stem-direction"
///                },
///                "voice": {
///                  "$ref": "#/$defs/voice-name"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "symbol": {
///            "$ref": "#/$defs/staff-symbol"
///          },
///          "type": {
///            "type": "string",
///            "const": "staff"
///          }
///        },
///        "additionalProperties": false
///      }
///    ]
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SystemLayoutContent(pub ::std::vec::Vec<SystemLayoutContentItem>);
impl ::std::ops::Deref for SystemLayoutContent {
    type Target = ::std::vec::Vec<SystemLayoutContentItem>;
    fn deref(&self) -> &::std::vec::Vec<SystemLayoutContentItem> {
        &self.0
    }
}
impl ::std::convert::From<SystemLayoutContent> for ::std::vec::Vec<SystemLayoutContentItem> {
    fn from(value: SystemLayoutContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemLayoutContent> for SystemLayoutContent {
    fn from(value: &SystemLayoutContent) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<SystemLayoutContentItem>> for SystemLayoutContent {
    fn from(value: ::std::vec::Vec<SystemLayoutContentItem>) -> Self {
        Self(value)
    }
}
///SystemLayoutContentItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "object",
///      "required": [
///        "content",
///        "type"
///      ],
///      "properties": {
///        "content": {
///          "$ref": "#/$defs/system-layout-content"
///        },
///        "label": {
///          "$ref": "#/$defs/staff-label"
///        },
///        "symbol": {
///          "$ref": "#/$defs/staff-symbol"
///        },
///        "type": {
///          "type": "string",
///          "const": "group"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "sources",
///        "type"
///      ],
///      "properties": {
///        "label": {
///          "$ref": "#/$defs/staff-label"
///        },
///        "labelref": {
///          "$ref": "#/$defs/staff-labelref"
///        },
///        "sources": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "part"
///            ],
///            "properties": {
///              "label": {
///                "$ref": "#/$defs/staff-label"
///              },
///              "labelref": {
///                "$ref": "#/$defs/staff-labelref"
///              },
///              "part": {
///                "$ref": "#/$defs/id"
///              },
///              "staff": {
///                "$ref": "#/$defs/staff-number"
///              },
///              "stem": {
///                "$ref": "#/$defs/stem-direction"
///              },
///              "voice": {
///                "$ref": "#/$defs/voice-name"
///              }
///            },
///            "additionalProperties": false
///          }
///        },
///        "symbol": {
///          "$ref": "#/$defs/staff-symbol"
///        },
///        "type": {
///          "type": "string",
///          "const": "staff"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum SystemLayoutContentItem {
    #[serde(rename = "group")]
    Group {
        content: SystemLayoutContent,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        label: ::std::option::Option<StaffLabel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        symbol: ::std::option::Option<StaffSymbol>,
    },
    #[serde(rename = "staff")]
    Staff {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        label: ::std::option::Option<StaffLabel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        labelref: ::std::option::Option<StaffLabelref>,
        sources: ::std::vec::Vec<SystemLayoutContentItemSourcesItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        symbol: ::std::option::Option<StaffSymbol>,
    },
}
impl ::std::convert::From<&Self> for SystemLayoutContentItem {
    fn from(value: &SystemLayoutContentItem) -> Self {
        value.clone()
    }
}
///SystemLayoutContentItemSourcesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "part"
///  ],
///  "properties": {
///    "label": {
///      "$ref": "#/$defs/staff-label"
///    },
///    "labelref": {
///      "$ref": "#/$defs/staff-labelref"
///    },
///    "part": {
///      "$ref": "#/$defs/id"
///    },
///    "staff": {
///      "$ref": "#/$defs/staff-number"
///    },
///    "stem": {
///      "$ref": "#/$defs/stem-direction"
///    },
///    "voice": {
///      "$ref": "#/$defs/voice-name"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SystemLayoutContentItemSourcesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<StaffLabel>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub labelref: ::std::option::Option<StaffLabelref>,
    pub part: Id,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stem: ::std::option::Option<StemDirection>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
}
impl ::std::convert::From<&SystemLayoutContentItemSourcesItem>
    for SystemLayoutContentItemSourcesItem
{
    fn from(value: &SystemLayoutContentItemSourcesItem) -> Self {
        value.clone()
    }
}
impl SystemLayoutContentItemSourcesItem {
    pub fn builder() -> builder::SystemLayoutContentItemSourcesItem {
        Default::default()
    }
}
///TupletDisplaySetting
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "noNumber",
///    "inner",
///    "both"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TupletDisplaySetting {
    #[serde(rename = "noNumber")]
    NoNumber,
    #[serde(rename = "inner")]
    Inner,
    #[serde(rename = "both")]
    Both,
}
impl ::std::convert::From<&Self> for TupletDisplaySetting {
    fn from(value: &TupletDisplaySetting) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TupletDisplaySetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NoNumber => write!(f, "noNumber"),
            Self::Inner => write!(f, "inner"),
            Self::Both => write!(f, "both"),
        }
    }
}
impl ::std::str::FromStr for TupletDisplaySetting {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "noNumber" => Ok(Self::NoNumber),
            "inner" => Ok(Self::Inner),
            "both" => Ok(Self::Both),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TupletDisplaySetting {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TupletDisplaySetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TupletDisplaySetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///UpOrDown
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "up",
///    "down"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpOrDown {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}
impl ::std::convert::From<&Self> for UpOrDown {
    fn from(value: &UpOrDown) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UpOrDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => write!(f, "up"),
            Self::Down => write!(f, "down"),
        }
    }
}
impl ::std::str::FromStr for UpOrDown {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpOrDown {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UpOrDown {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpOrDown {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///VoiceName
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct VoiceName(pub ::std::string::String);
impl ::std::ops::Deref for VoiceName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceName> for ::std::string::String {
    fn from(value: VoiceName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceName> for VoiceName {
    fn from(value: &VoiceName) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for VoiceName {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for VoiceName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for VoiceName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BeamListItem {
        events: ::std::result::Result<::std::vec::Vec<super::Id>, ::std::string::String>,
        hooks: ::std::result::Result<
            ::std::vec::Vec<super::BeamListItemHooksItem>,
            ::std::string::String,
        >,
        inner: ::std::result::Result<::std::option::Option<super::BeamList>, ::std::string::String>,
    }
    impl ::std::default::Default for BeamListItem {
        fn default() -> Self {
            Self {
                events: Err("no value supplied for events".to_string()),
                hooks: Ok(Default::default()),
                inner: Ok(Default::default()),
            }
        }
    }
    impl BeamListItem {
        pub fn events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.events = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for events: {}", e));
            self
        }
        pub fn hooks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BeamListItemHooksItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.hooks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hooks: {}", e));
            self
        }
        pub fn inner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BeamList>>,
            T::Error: ::std::fmt::Display,
        {
            self.inner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inner: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BeamListItem> for super::BeamListItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BeamListItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                events: value.events?,
                hooks: value.hooks?,
                inner: value.inner?,
            })
        }
    }
    impl ::std::convert::From<super::BeamListItem> for BeamListItem {
        fn from(value: super::BeamListItem) -> Self {
            Self {
                events: Ok(value.events),
                hooks: Ok(value.hooks),
                inner: Ok(value.inner),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BeamListItemHooksItem {
        direction:
            ::std::result::Result<super::BeamListItemHooksItemDirection, ::std::string::String>,
        event: ::std::result::Result<super::Id, ::std::string::String>,
    }
    impl ::std::default::Default for BeamListItemHooksItem {
        fn default() -> Self {
            Self {
                direction: Err("no value supplied for direction".to_string()),
                event: Err("no value supplied for event".to_string()),
            }
        }
    }
    impl BeamListItemHooksItem {
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BeamListItemHooksItemDirection>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {}", e));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BeamListItemHooksItem> for super::BeamListItemHooksItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BeamListItemHooksItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                event: value.event?,
            })
        }
    }
    impl ::std::convert::From<super::BeamListItemHooksItem> for BeamListItemHooksItem {
        fn from(value: super::BeamListItemHooksItem) -> Self {
            Self {
                direction: Ok(value.direction),
                event: Ok(value.event),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Event {
        duration:
            ::std::result::Result<::std::option::Option<super::NoteValue>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        lyrics:
            ::std::result::Result<::std::option::Option<super::EventLyrics>, ::std::string::String>,
        markings: ::std::result::Result<
            ::std::option::Option<super::EventMarkings>,
            ::std::string::String,
        >,
        measure: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        notes: ::std::result::Result<::std::vec::Vec<super::EventNotesItem>, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        rest: ::std::result::Result<::std::option::Option<super::EventRest>, ::std::string::String>,
        slurs: ::std::result::Result<::std::vec::Vec<super::EventSlursItem>, ::std::string::String>,
        smufl_font:
            ::std::result::Result<::std::option::Option<super::SmuflFont>, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        stem_direction: ::std::result::Result<
            ::std::option::Option<super::StemDirection>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Event {
        fn default() -> Self {
            Self {
                duration: Ok(Default::default()),
                id: Ok(Default::default()),
                lyrics: Ok(Default::default()),
                markings: Ok(Default::default()),
                measure: Ok(Default::default()),
                notes: Ok(Default::default()),
                orient: Ok(Default::default()),
                rest: Ok(Default::default()),
                slurs: Ok(Default::default()),
                smufl_font: Ok(Default::default()),
                staff: Ok(Default::default()),
                stem_direction: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl Event {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NoteValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn lyrics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventLyrics>>,
            T::Error: ::std::fmt::Display,
        {
            self.lyrics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lyrics: {}", e));
            self
        }
        pub fn markings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkings>>,
            T::Error: ::std::fmt::Display,
        {
            self.markings = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for markings: {}", e));
            self
        }
        pub fn measure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.measure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measure: {}", e));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EventNotesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {}", e));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {}", e));
            self
        }
        pub fn rest<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventRest>>,
            T::Error: ::std::fmt::Display,
        {
            self.rest = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rest: {}", e));
            self
        }
        pub fn slurs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EventSlursItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.slurs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for slurs: {}", e));
            self
        }
        pub fn smufl_font<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflFont>>,
            T::Error: ::std::fmt::Display,
        {
            self.smufl_font = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for smufl_font: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
        pub fn stem_direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StemDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.stem_direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stem_direction: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Event> for super::Event {
        type Error = super::error::ConversionError;
        fn try_from(value: Event) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                id: value.id?,
                lyrics: value.lyrics?,
                markings: value.markings?,
                measure: value.measure?,
                notes: value.notes?,
                orient: value.orient?,
                rest: value.rest?,
                slurs: value.slurs?,
                smufl_font: value.smufl_font?,
                staff: value.staff?,
                stem_direction: value.stem_direction?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Event> for Event {
        fn from(value: super::Event) -> Self {
            Self {
                duration: Ok(value.duration),
                id: Ok(value.id),
                lyrics: Ok(value.lyrics),
                markings: Ok(value.markings),
                measure: Ok(value.measure),
                notes: Ok(value.notes),
                orient: Ok(value.orient),
                rest: Ok(value.rest),
                slurs: Ok(value.slurs),
                smufl_font: Ok(value.smufl_font),
                staff: Ok(value.staff),
                stem_direction: Ok(value.stem_direction),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventLyrics {
        lines: ::std::result::Result<
            ::std::collections::HashMap<super::EventLyricsLinesKey, super::EventLyricsLinesValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventLyrics {
        fn default() -> Self {
            Self {
                lines: Ok(Default::default()),
            }
        }
    }
    impl EventLyrics {
        pub fn lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::EventLyricsLinesKey,
                    super::EventLyricsLinesValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lines: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventLyrics> for super::EventLyrics {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventLyrics,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                lines: value.lines?,
            })
        }
    }
    impl ::std::convert::From<super::EventLyrics> for EventLyrics {
        fn from(value: super::EventLyrics) -> Self {
            Self {
                lines: Ok(value.lines),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventLyricsLinesValue {
        text: ::std::result::Result<super::String, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::EventLyricsLinesValueType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventLyricsLinesValue {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl EventLyricsLinesValue {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventLyricsLinesValueType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventLyricsLinesValue> for super::EventLyricsLinesValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventLyricsLinesValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EventLyricsLinesValue> for EventLyricsLinesValue {
        fn from(value: super::EventLyricsLinesValue) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkings {
        accent: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsAccent>,
            ::std::string::String,
        >,
        breath: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsBreath>,
            ::std::string::String,
        >,
        soft_accent: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsSoftAccent>,
            ::std::string::String,
        >,
        spiccato: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsSpiccato>,
            ::std::string::String,
        >,
        staccatissimo: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsStaccatissimo>,
            ::std::string::String,
        >,
        staccato: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsStaccato>,
            ::std::string::String,
        >,
        stress: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsStress>,
            ::std::string::String,
        >,
        strong_accent: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsStrongAccent>,
            ::std::string::String,
        >,
        tenuto: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsTenuto>,
            ::std::string::String,
        >,
        tremolo: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsTremolo>,
            ::std::string::String,
        >,
        unstress: ::std::result::Result<
            ::std::option::Option<super::EventMarkingsUnstress>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventMarkings {
        fn default() -> Self {
            Self {
                accent: Ok(Default::default()),
                breath: Ok(Default::default()),
                soft_accent: Ok(Default::default()),
                spiccato: Ok(Default::default()),
                staccatissimo: Ok(Default::default()),
                staccato: Ok(Default::default()),
                stress: Ok(Default::default()),
                strong_accent: Ok(Default::default()),
                tenuto: Ok(Default::default()),
                tremolo: Ok(Default::default()),
                unstress: Ok(Default::default()),
            }
        }
    }
    impl EventMarkings {
        pub fn accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsAccent>>,
            T::Error: ::std::fmt::Display,
        {
            self.accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accent: {}", e));
            self
        }
        pub fn breath<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsBreath>>,
            T::Error: ::std::fmt::Display,
        {
            self.breath = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for breath: {}", e));
            self
        }
        pub fn soft_accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsSoftAccent>>,
            T::Error: ::std::fmt::Display,
        {
            self.soft_accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for soft_accent: {}", e));
            self
        }
        pub fn spiccato<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsSpiccato>>,
            T::Error: ::std::fmt::Display,
        {
            self.spiccato = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spiccato: {}", e));
            self
        }
        pub fn staccatissimo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsStaccatissimo>>,
            T::Error: ::std::fmt::Display,
        {
            self.staccatissimo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staccatissimo: {}", e));
            self
        }
        pub fn staccato<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsStaccato>>,
            T::Error: ::std::fmt::Display,
        {
            self.staccato = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staccato: {}", e));
            self
        }
        pub fn stress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsStress>>,
            T::Error: ::std::fmt::Display,
        {
            self.stress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stress: {}", e));
            self
        }
        pub fn strong_accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsStrongAccent>>,
            T::Error: ::std::fmt::Display,
        {
            self.strong_accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strong_accent: {}", e));
            self
        }
        pub fn tenuto<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsTenuto>>,
            T::Error: ::std::fmt::Display,
        {
            self.tenuto = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenuto: {}", e));
            self
        }
        pub fn tremolo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsTremolo>>,
            T::Error: ::std::fmt::Display,
        {
            self.tremolo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tremolo: {}", e));
            self
        }
        pub fn unstress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkingsUnstress>>,
            T::Error: ::std::fmt::Display,
        {
            self.unstress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unstress: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventMarkings> for super::EventMarkings {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventMarkings,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accent: value.accent?,
                breath: value.breath?,
                soft_accent: value.soft_accent?,
                spiccato: value.spiccato?,
                staccatissimo: value.staccatissimo?,
                staccato: value.staccato?,
                stress: value.stress?,
                strong_accent: value.strong_accent?,
                tenuto: value.tenuto?,
                tremolo: value.tremolo?,
                unstress: value.unstress?,
            })
        }
    }
    impl ::std::convert::From<super::EventMarkings> for EventMarkings {
        fn from(value: super::EventMarkings) -> Self {
            Self {
                accent: Ok(value.accent),
                breath: Ok(value.breath),
                soft_accent: Ok(value.soft_accent),
                spiccato: Ok(value.spiccato),
                staccatissimo: Ok(value.staccatissimo),
                staccato: Ok(value.staccato),
                stress: Ok(value.stress),
                strong_accent: Ok(value.strong_accent),
                tenuto: Ok(value.tenuto),
                tremolo: Ok(value.tremolo),
                unstress: Ok(value.unstress),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsAccent {
        pointing:
            ::std::result::Result<::std::option::Option<super::UpOrDown>, ::std::string::String>,
    }
    impl ::std::default::Default for EventMarkingsAccent {
        fn default() -> Self {
            Self {
                pointing: Ok(Default::default()),
            }
        }
    }
    impl EventMarkingsAccent {
        pub fn pointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UpOrDown>>,
            T::Error: ::std::fmt::Display,
        {
            self.pointing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pointing: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventMarkingsAccent> for super::EventMarkingsAccent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventMarkingsAccent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pointing: value.pointing?,
            })
        }
    }
    impl ::std::convert::From<super::EventMarkingsAccent> for EventMarkingsAccent {
        fn from(value: super::EventMarkingsAccent) -> Self {
            Self {
                pointing: Ok(value.pointing),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsBreath {
        symbol: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventMarkingsBreath {
        fn default() -> Self {
            Self {
                symbol: Ok(Default::default()),
            }
        }
    }
    impl EventMarkingsBreath {
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventMarkingsBreath> for super::EventMarkingsBreath {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventMarkingsBreath,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                symbol: value.symbol?,
            })
        }
    }
    impl ::std::convert::From<super::EventMarkingsBreath> for EventMarkingsBreath {
        fn from(value: super::EventMarkingsBreath) -> Self {
            Self {
                symbol: Ok(value.symbol),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsSoftAccent {}
    impl ::std::default::Default for EventMarkingsSoftAccent {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventMarkingsSoftAccent {}
    impl ::std::convert::TryFrom<EventMarkingsSoftAccent> for super::EventMarkingsSoftAccent {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventMarkingsSoftAccent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventMarkingsSoftAccent> for EventMarkingsSoftAccent {
        fn from(_value: super::EventMarkingsSoftAccent) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsSpiccato {}
    impl ::std::default::Default for EventMarkingsSpiccato {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventMarkingsSpiccato {}
    impl ::std::convert::TryFrom<EventMarkingsSpiccato> for super::EventMarkingsSpiccato {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventMarkingsSpiccato,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventMarkingsSpiccato> for EventMarkingsSpiccato {
        fn from(_value: super::EventMarkingsSpiccato) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsStaccatissimo {}
    impl ::std::default::Default for EventMarkingsStaccatissimo {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventMarkingsStaccatissimo {}
    impl ::std::convert::TryFrom<EventMarkingsStaccatissimo> for super::EventMarkingsStaccatissimo {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventMarkingsStaccatissimo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventMarkingsStaccatissimo> for EventMarkingsStaccatissimo {
        fn from(_value: super::EventMarkingsStaccatissimo) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsStaccato {}
    impl ::std::default::Default for EventMarkingsStaccato {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventMarkingsStaccato {}
    impl ::std::convert::TryFrom<EventMarkingsStaccato> for super::EventMarkingsStaccato {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventMarkingsStaccato,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventMarkingsStaccato> for EventMarkingsStaccato {
        fn from(_value: super::EventMarkingsStaccato) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsStress {}
    impl ::std::default::Default for EventMarkingsStress {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventMarkingsStress {}
    impl ::std::convert::TryFrom<EventMarkingsStress> for super::EventMarkingsStress {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventMarkingsStress,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventMarkingsStress> for EventMarkingsStress {
        fn from(_value: super::EventMarkingsStress) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsStrongAccent {
        pointing:
            ::std::result::Result<::std::option::Option<super::UpOrDown>, ::std::string::String>,
    }
    impl ::std::default::Default for EventMarkingsStrongAccent {
        fn default() -> Self {
            Self {
                pointing: Ok(Default::default()),
            }
        }
    }
    impl EventMarkingsStrongAccent {
        pub fn pointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UpOrDown>>,
            T::Error: ::std::fmt::Display,
        {
            self.pointing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pointing: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventMarkingsStrongAccent> for super::EventMarkingsStrongAccent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventMarkingsStrongAccent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pointing: value.pointing?,
            })
        }
    }
    impl ::std::convert::From<super::EventMarkingsStrongAccent> for EventMarkingsStrongAccent {
        fn from(value: super::EventMarkingsStrongAccent) -> Self {
            Self {
                pointing: Ok(value.pointing),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsTenuto {}
    impl ::std::default::Default for EventMarkingsTenuto {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventMarkingsTenuto {}
    impl ::std::convert::TryFrom<EventMarkingsTenuto> for super::EventMarkingsTenuto {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventMarkingsTenuto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventMarkingsTenuto> for EventMarkingsTenuto {
        fn from(_value: super::EventMarkingsTenuto) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsTremolo {
        marks: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
    }
    impl ::std::default::Default for EventMarkingsTremolo {
        fn default() -> Self {
            Self {
                marks: Err("no value supplied for marks".to_string()),
            }
        }
    }
    impl EventMarkingsTremolo {
        pub fn marks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.marks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marks: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventMarkingsTremolo> for super::EventMarkingsTremolo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventMarkingsTremolo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                marks: value.marks?,
            })
        }
    }
    impl ::std::convert::From<super::EventMarkingsTremolo> for EventMarkingsTremolo {
        fn from(value: super::EventMarkingsTremolo) -> Self {
            Self {
                marks: Ok(value.marks),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkingsUnstress {}
    impl ::std::default::Default for EventMarkingsUnstress {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventMarkingsUnstress {}
    impl ::std::convert::TryFrom<EventMarkingsUnstress> for super::EventMarkingsUnstress {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventMarkingsUnstress,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventMarkingsUnstress> for EventMarkingsUnstress {
        fn from(_value: super::EventMarkingsUnstress) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventNotesItem {
        accidental_display: ::std::result::Result<
            ::std::option::Option<super::EventNotesItemAccidentalDisplay>,
            ::std::string::String,
        >,
        class:
            ::std::result::Result<::std::option::Option<super::StyleClass>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        perform: ::std::result::Result<
            ::std::option::Option<super::EventNotesItemPerform>,
            ::std::string::String,
        >,
        pitch: ::std::result::Result<super::EventNotesItemPitch, ::std::string::String>,
        smufl_font:
            ::std::result::Result<::std::option::Option<super::SmuflFont>, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        ties: ::std::result::Result<
            ::std::vec::Vec<super::EventNotesItemTiesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventNotesItem {
        fn default() -> Self {
            Self {
                accidental_display: Ok(Default::default()),
                class: Ok(Default::default()),
                id: Ok(Default::default()),
                perform: Ok(Default::default()),
                pitch: Err("no value supplied for pitch".to_string()),
                smufl_font: Ok(Default::default()),
                staff: Ok(Default::default()),
                ties: Ok(Default::default()),
            }
        }
    }
    impl EventNotesItem {
        pub fn accidental_display<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::EventNotesItemAccidentalDisplay>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.accidental_display = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for accidental_display: {}",
                    e
                )
            });
            self
        }
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StyleClass>>,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn perform<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventNotesItemPerform>>,
            T::Error: ::std::fmt::Display,
        {
            self.perform = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for perform: {}", e));
            self
        }
        pub fn pitch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventNotesItemPitch>,
            T::Error: ::std::fmt::Display,
        {
            self.pitch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pitch: {}", e));
            self
        }
        pub fn smufl_font<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflFont>>,
            T::Error: ::std::fmt::Display,
        {
            self.smufl_font = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for smufl_font: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
        pub fn ties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EventNotesItemTiesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.ties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ties: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventNotesItem> for super::EventNotesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventNotesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accidental_display: value.accidental_display?,
                class: value.class?,
                id: value.id?,
                perform: value.perform?,
                pitch: value.pitch?,
                smufl_font: value.smufl_font?,
                staff: value.staff?,
                ties: value.ties?,
            })
        }
    }
    impl ::std::convert::From<super::EventNotesItem> for EventNotesItem {
        fn from(value: super::EventNotesItem) -> Self {
            Self {
                accidental_display: Ok(value.accidental_display),
                class: Ok(value.class),
                id: Ok(value.id),
                perform: Ok(value.perform),
                pitch: Ok(value.pitch),
                smufl_font: Ok(value.smufl_font),
                staff: Ok(value.staff),
                ties: Ok(value.ties),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventNotesItemAccidentalDisplay {
        enclosure: ::std::result::Result<
            ::std::option::Option<super::EventNotesItemAccidentalDisplayEnclosure>,
            ::std::string::String,
        >,
        force: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        show: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for EventNotesItemAccidentalDisplay {
        fn default() -> Self {
            Self {
                enclosure: Ok(Default::default()),
                force: Ok(Default::default()),
                show: Err("no value supplied for show".to_string()),
            }
        }
    }
    impl EventNotesItemAccidentalDisplay {
        pub fn enclosure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::EventNotesItemAccidentalDisplayEnclosure>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.enclosure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enclosure: {}", e));
            self
        }
        pub fn force<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.force = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for force: {}", e));
            self
        }
        pub fn show<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.show = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventNotesItemAccidentalDisplay>
        for super::EventNotesItemAccidentalDisplay
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventNotesItemAccidentalDisplay,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enclosure: value.enclosure?,
                force: value.force?,
                show: value.show?,
            })
        }
    }
    impl ::std::convert::From<super::EventNotesItemAccidentalDisplay>
        for EventNotesItemAccidentalDisplay
    {
        fn from(value: super::EventNotesItemAccidentalDisplay) -> Self {
            Self {
                enclosure: Ok(value.enclosure),
                force: Ok(value.force),
                show: Ok(value.show),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventNotesItemAccidentalDisplayEnclosure {
        symbol: ::std::result::Result<
            super::EventNotesItemAccidentalDisplayEnclosureSymbol,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventNotesItemAccidentalDisplayEnclosure {
        fn default() -> Self {
            Self {
                symbol: Err("no value supplied for symbol".to_string()),
            }
        }
    }
    impl EventNotesItemAccidentalDisplayEnclosure {
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventNotesItemAccidentalDisplayEnclosureSymbol>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventNotesItemAccidentalDisplayEnclosure>
        for super::EventNotesItemAccidentalDisplayEnclosure
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventNotesItemAccidentalDisplayEnclosure,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                symbol: value.symbol?,
            })
        }
    }
    impl ::std::convert::From<super::EventNotesItemAccidentalDisplayEnclosure>
        for EventNotesItemAccidentalDisplayEnclosure
    {
        fn from(value: super::EventNotesItemAccidentalDisplayEnclosure) -> Self {
            Self {
                symbol: Ok(value.symbol),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventNotesItemPerform {}
    impl ::std::default::Default for EventNotesItemPerform {
        fn default() -> Self {
            Self {}
        }
    }
    impl EventNotesItemPerform {}
    impl ::std::convert::TryFrom<EventNotesItemPerform> for super::EventNotesItemPerform {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: EventNotesItemPerform,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::EventNotesItemPerform> for EventNotesItemPerform {
        fn from(_value: super::EventNotesItemPerform) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventNotesItemPitch {
        alter: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        octave: ::std::result::Result<i64, ::std::string::String>,
        step: ::std::result::Result<super::EventNotesItemPitchStep, ::std::string::String>,
    }
    impl ::std::default::Default for EventNotesItemPitch {
        fn default() -> Self {
            Self {
                alter: Ok(Default::default()),
                octave: Err("no value supplied for octave".to_string()),
                step: Err("no value supplied for step".to_string()),
            }
        }
    }
    impl EventNotesItemPitch {
        pub fn alter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.alter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alter: {}", e));
            self
        }
        pub fn octave<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.octave = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for octave: {}", e));
            self
        }
        pub fn step<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventNotesItemPitchStep>,
            T::Error: ::std::fmt::Display,
        {
            self.step = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventNotesItemPitch> for super::EventNotesItemPitch {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventNotesItemPitch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alter: value.alter?,
                octave: value.octave?,
                step: value.step?,
            })
        }
    }
    impl ::std::convert::From<super::EventNotesItemPitch> for EventNotesItemPitch {
        fn from(value: super::EventNotesItemPitch) -> Self {
            Self {
                alter: Ok(value.alter),
                octave: Ok(value.octave),
                step: Ok(value.step),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventNotesItemTiesItem {
        lv: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        side: ::std::result::Result<::std::option::Option<super::SlurSide>, ::std::string::String>,
        target: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
    }
    impl ::std::default::Default for EventNotesItemTiesItem {
        fn default() -> Self {
            Self {
                lv: Ok(Default::default()),
                side: Ok(Default::default()),
                target: Ok(Default::default()),
            }
        }
    }
    impl EventNotesItemTiesItem {
        pub fn lv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.lv = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lv: {}", e));
            self
        }
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SlurSide>>,
            T::Error: ::std::fmt::Display,
        {
            self.side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {}", e));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventNotesItemTiesItem> for super::EventNotesItemTiesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventNotesItemTiesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                lv: value.lv?,
                side: value.side?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::EventNotesItemTiesItem> for EventNotesItemTiesItem {
        fn from(value: super::EventNotesItemTiesItem) -> Self {
            Self {
                lv: Ok(value.lv),
                side: Ok(value.side),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventRest {
        staff_position: ::std::result::Result<
            ::std::option::Option<super::StaffPosition>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventRest {
        fn default() -> Self {
            Self {
                staff_position: Ok(Default::default()),
            }
        }
    }
    impl EventRest {
        pub fn staff_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffPosition>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_position: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventRest> for super::EventRest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventRest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                staff_position: value.staff_position?,
            })
        }
    }
    impl ::std::convert::From<super::EventRest> for EventRest {
        fn from(value: super::EventRest) -> Self {
            Self {
                staff_position: Ok(value.staff_position),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventSlursItem {
        end_note: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        line_type: ::std::result::Result<
            ::std::option::Option<super::EventSlursItemLineType>,
            ::std::string::String,
        >,
        side: ::std::result::Result<::std::option::Option<super::SlurSide>, ::std::string::String>,
        side_end:
            ::std::result::Result<::std::option::Option<super::SlurSide>, ::std::string::String>,
        start_note: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        target: ::std::result::Result<super::Id, ::std::string::String>,
    }
    impl ::std::default::Default for EventSlursItem {
        fn default() -> Self {
            Self {
                end_note: Ok(Default::default()),
                line_type: Ok(Default::default()),
                side: Ok(Default::default()),
                side_end: Ok(Default::default()),
                start_note: Ok(Default::default()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl EventSlursItem {
        pub fn end_note<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_note = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_note: {}", e));
            self
        }
        pub fn line_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventSlursItemLineType>>,
            T::Error: ::std::fmt::Display,
        {
            self.line_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_type: {}", e));
            self
        }
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SlurSide>>,
            T::Error: ::std::fmt::Display,
        {
            self.side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {}", e));
            self
        }
        pub fn side_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SlurSide>>,
            T::Error: ::std::fmt::Display,
        {
            self.side_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side_end: {}", e));
            self
        }
        pub fn start_note<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_note = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_note: {}", e));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EventSlursItem> for super::EventSlursItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventSlursItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_note: value.end_note?,
                line_type: value.line_type?,
                side: value.side?,
                side_end: value.side_end?,
                start_note: value.start_note?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::EventSlursItem> for EventSlursItem {
        fn from(value: super::EventSlursItem) -> Self {
            Self {
                end_note: Ok(value.end_note),
                line_type: Ok(value.line_type),
                side: Ok(value.side),
                side_end: Ok(value.side_end),
                start_note: Ok(value.start_note),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MeasureRhythmicPosition {
        measure: ::std::result::Result<super::MeasureNumber, ::std::string::String>,
        position: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
    }
    impl ::std::default::Default for MeasureRhythmicPosition {
        fn default() -> Self {
            Self {
                measure: Err("no value supplied for measure".to_string()),
                position: Err("no value supplied for position".to_string()),
            }
        }
    }
    impl MeasureRhythmicPosition {
        pub fn measure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.measure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measure: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MeasureRhythmicPosition> for super::MeasureRhythmicPosition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MeasureRhythmicPosition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                measure: value.measure?,
                position: value.position?,
            })
        }
    }
    impl ::std::convert::From<super::MeasureRhythmicPosition> for MeasureRhythmicPosition {
        fn from(value: super::MeasureRhythmicPosition) -> Self {
            Self {
                measure: Ok(value.measure),
                position: Ok(value.position),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocument {
        global: ::std::result::Result<super::MnxDocumentGlobal, ::std::string::String>,
        layouts: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentLayoutsItem>,
            ::std::string::String,
        >,
        mnx: ::std::result::Result<super::MnxDocumentMnx, ::std::string::String>,
        parts: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentPartsItem>,
            ::std::string::String,
        >,
        scores: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentScoresItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocument {
        fn default() -> Self {
            Self {
                global: Err("no value supplied for global".to_string()),
                layouts: Ok(Default::default()),
                mnx: Err("no value supplied for mnx".to_string()),
                parts: Err("no value supplied for parts".to_string()),
                scores: Ok(Default::default()),
            }
        }
    }
    impl MnxDocument {
        pub fn global<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentGlobal>,
            T::Error: ::std::fmt::Display,
        {
            self.global = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for global: {}", e));
            self
        }
        pub fn layouts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MnxDocumentLayoutsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.layouts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layouts: {}", e));
            self
        }
        pub fn mnx<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentMnx>,
            T::Error: ::std::fmt::Display,
        {
            self.mnx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mnx: {}", e));
            self
        }
        pub fn parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MnxDocumentPartsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parts: {}", e));
            self
        }
        pub fn scores<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MnxDocumentScoresItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scores: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocument> for super::MnxDocument {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                global: value.global?,
                layouts: value.layouts?,
                mnx: value.mnx?,
                parts: value.parts?,
                scores: value.scores?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocument> for MnxDocument {
        fn from(value: super::MnxDocument) -> Self {
            Self {
                global: Ok(value.global),
                layouts: Ok(value.layouts),
                mnx: Ok(value.mnx),
                parts: Ok(value.parts),
                scores: Ok(value.scores),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobal {
        lyrics: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalLyrics>,
            ::std::string::String,
        >,
        measures: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentGlobalMeasuresItem>,
            ::std::string::String,
        >,
        styles: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentGlobalStylesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentGlobal {
        fn default() -> Self {
            Self {
                lyrics: Ok(Default::default()),
                measures: Err("no value supplied for measures".to_string()),
                styles: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentGlobal {
        pub fn lyrics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MnxDocumentGlobalLyrics>>,
            T::Error: ::std::fmt::Display,
        {
            self.lyrics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lyrics: {}", e));
            self
        }
        pub fn measures<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MnxDocumentGlobalMeasuresItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.measures = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measures: {}", e));
            self
        }
        pub fn styles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MnxDocumentGlobalStylesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.styles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for styles: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobal> for super::MnxDocumentGlobal {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobal,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                lyrics: value.lyrics?,
                measures: value.measures?,
                styles: value.styles?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobal> for MnxDocumentGlobal {
        fn from(value: super::MnxDocumentGlobal) -> Self {
            Self {
                lyrics: Ok(value.lyrics),
                measures: Ok(value.measures),
                styles: Ok(value.styles),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalLyrics {
        line_metadata: ::std::result::Result<
            ::std::collections::HashMap<
                super::MnxDocumentGlobalLyricsLineMetadataKey,
                super::MnxDocumentGlobalLyricsLineMetadataValue,
            >,
            ::std::string::String,
        >,
        line_order:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalLyrics {
        fn default() -> Self {
            Self {
                line_metadata: Ok(Default::default()),
                line_order: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentGlobalLyrics {
        pub fn line_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::MnxDocumentGlobalLyricsLineMetadataKey,
                    super::MnxDocumentGlobalLyricsLineMetadataValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.line_metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_metadata: {}", e));
            self
        }
        pub fn line_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.line_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_order: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalLyrics> for super::MnxDocumentGlobalLyrics {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalLyrics,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                line_metadata: value.line_metadata?,
                line_order: value.line_order?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalLyrics> for MnxDocumentGlobalLyrics {
        fn from(value: super::MnxDocumentGlobalLyrics) -> Self {
            Self {
                line_metadata: Ok(value.line_metadata),
                line_order: Ok(value.line_order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalLyricsLineMetadataValue {
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        lang: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentGlobalLyricsLineMetadataValue {
        fn default() -> Self {
            Self {
                label: Ok(Default::default()),
                lang: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentGlobalLyricsLineMetadataValue {
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn lang<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lang: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalLyricsLineMetadataValue>
        for super::MnxDocumentGlobalLyricsLineMetadataValue
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalLyricsLineMetadataValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                lang: value.lang?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalLyricsLineMetadataValue>
        for MnxDocumentGlobalLyricsLineMetadataValue
    {
        fn from(value: super::MnxDocumentGlobalLyricsLineMetadataValue) -> Self {
            Self {
                label: Ok(value.label),
                lang: Ok(value.lang),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItem {
        barline: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemBarline>,
            ::std::string::String,
        >,
        ending: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemEnding>,
            ::std::string::String,
        >,
        fine: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemFine>,
            ::std::string::String,
        >,
        index: ::std::result::Result<
            ::std::option::Option<super::MeasureNumber>,
            ::std::string::String,
        >,
        jump: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemJump>,
            ::std::string::String,
        >,
        key: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemKey>,
            ::std::string::String,
        >,
        number: ::std::result::Result<
            ::std::option::Option<super::MeasureNumber>,
            ::std::string::String,
        >,
        repeat_end: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemRepeatEnd>,
            ::std::string::String,
        >,
        repeat_start: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemRepeatStart>,
            ::std::string::String,
        >,
        segno: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemSegno>,
            ::std::string::String,
        >,
        tempos: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentGlobalMeasuresItemTemposItem>,
            ::std::string::String,
        >,
        time: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentGlobalMeasuresItemTime>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItem {
        fn default() -> Self {
            Self {
                barline: Ok(Default::default()),
                ending: Ok(Default::default()),
                fine: Ok(Default::default()),
                index: Ok(Default::default()),
                jump: Ok(Default::default()),
                key: Ok(Default::default()),
                number: Ok(Default::default()),
                repeat_end: Ok(Default::default()),
                repeat_start: Ok(Default::default()),
                segno: Ok(Default::default()),
                tempos: Ok(Default::default()),
                time: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItem {
        pub fn barline<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemBarline>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.barline = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for barline: {}", e));
            self
        }
        pub fn ending<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemEnding>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ending = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ending: {}", e));
            self
        }
        pub fn fine<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemFine>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.fine = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fine: {}", e));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MeasureNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn jump<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemJump>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.jump = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jump: {}", e));
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemKey>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MeasureNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {}", e));
            self
        }
        pub fn repeat_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemRepeatEnd>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.repeat_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat_end: {}", e));
            self
        }
        pub fn repeat_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemRepeatStart>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.repeat_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat_start: {}", e));
            self
        }
        pub fn segno<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemSegno>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.segno = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segno: {}", e));
            self
        }
        pub fn tempos<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentGlobalMeasuresItemTemposItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tempos = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tempos: {}", e));
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MnxDocumentGlobalMeasuresItemTime>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItem>
        for super::MnxDocumentGlobalMeasuresItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                barline: value.barline?,
                ending: value.ending?,
                fine: value.fine?,
                index: value.index?,
                jump: value.jump?,
                key: value.key?,
                number: value.number?,
                repeat_end: value.repeat_end?,
                repeat_start: value.repeat_start?,
                segno: value.segno?,
                tempos: value.tempos?,
                time: value.time?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItem> for MnxDocumentGlobalMeasuresItem {
        fn from(value: super::MnxDocumentGlobalMeasuresItem) -> Self {
            Self {
                barline: Ok(value.barline),
                ending: Ok(value.ending),
                fine: Ok(value.fine),
                index: Ok(value.index),
                jump: Ok(value.jump),
                key: Ok(value.key),
                number: Ok(value.number),
                repeat_end: Ok(value.repeat_end),
                repeat_start: Ok(value.repeat_start),
                segno: Ok(value.segno),
                tempos: Ok(value.tempos),
                time: Ok(value.time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemBarline {
        type_: ::std::result::Result<
            super::MnxDocumentGlobalMeasuresItemBarlineType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemBarline {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemBarline {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentGlobalMeasuresItemBarlineType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemBarline>
        for super::MnxDocumentGlobalMeasuresItemBarline
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemBarline,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemBarline>
        for MnxDocumentGlobalMeasuresItemBarline
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemBarline) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemEnding {
        class:
            ::std::result::Result<::std::option::Option<super::StyleClass>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        duration: ::std::result::Result<i64, ::std::string::String>,
        numbers: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        open: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemEnding {
        fn default() -> Self {
            Self {
                class: Ok(Default::default()),
                color: Ok(Default::default()),
                duration: Err("no value supplied for duration".to_string()),
                numbers: Ok(Default::default()),
                open: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemEnding {
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StyleClass>>,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn numbers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.numbers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for numbers: {}", e));
            self
        }
        pub fn open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemEnding>
        for super::MnxDocumentGlobalMeasuresItemEnding
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemEnding,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                class: value.class?,
                color: value.color?,
                duration: value.duration?,
                numbers: value.numbers?,
                open: value.open?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemEnding>
        for MnxDocumentGlobalMeasuresItemEnding
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemEnding) -> Self {
            Self {
                class: Ok(value.class),
                color: Ok(value.color),
                duration: Ok(value.duration),
                numbers: Ok(value.numbers),
                open: Ok(value.open),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemFine {
        class:
            ::std::result::Result<::std::option::Option<super::StyleClass>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        location: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemFine {
        fn default() -> Self {
            Self {
                class: Ok(Default::default()),
                color: Ok(Default::default()),
                location: Err("no value supplied for location".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemFine {
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StyleClass>>,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemFine>
        for super::MnxDocumentGlobalMeasuresItemFine
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemFine,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                class: value.class?,
                color: value.color?,
                location: value.location?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemFine>
        for MnxDocumentGlobalMeasuresItemFine
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemFine) -> Self {
            Self {
                class: Ok(value.class),
                color: Ok(value.color),
                location: Ok(value.location),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemJump {
        location: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        type_: ::std::result::Result<
            super::MnxDocumentGlobalMeasuresItemJumpType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemJump {
        fn default() -> Self {
            Self {
                location: Err("no value supplied for location".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemJump {
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentGlobalMeasuresItemJumpType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemJump>
        for super::MnxDocumentGlobalMeasuresItemJump
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemJump,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                location: value.location?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemJump>
        for MnxDocumentGlobalMeasuresItemJump
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemJump) -> Self {
            Self {
                location: Ok(value.location),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemKey {
        class:
            ::std::result::Result<::std::option::Option<super::StyleClass>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        fifths: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemKey {
        fn default() -> Self {
            Self {
                class: Ok(Default::default()),
                color: Ok(Default::default()),
                fifths: Err("no value supplied for fifths".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemKey {
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StyleClass>>,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn fifths<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.fifths = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fifths: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemKey>
        for super::MnxDocumentGlobalMeasuresItemKey
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemKey,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                class: value.class?,
                color: value.color?,
                fifths: value.fifths?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemKey>
        for MnxDocumentGlobalMeasuresItemKey
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemKey) -> Self {
            Self {
                class: Ok(value.class),
                color: Ok(value.color),
                fifths: Ok(value.fifths),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemRepeatEnd {
        times: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemRepeatEnd {
        fn default() -> Self {
            Self {
                times: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemRepeatEnd {
        pub fn times<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.times = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemRepeatEnd>
        for super::MnxDocumentGlobalMeasuresItemRepeatEnd
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemRepeatEnd,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                times: value.times?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemRepeatEnd>
        for MnxDocumentGlobalMeasuresItemRepeatEnd
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemRepeatEnd) -> Self {
            Self {
                times: Ok(value.times),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemRepeatStart {}
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemRepeatStart {
        fn default() -> Self {
            Self {}
        }
    }
    impl MnxDocumentGlobalMeasuresItemRepeatStart {}
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemRepeatStart>
        for super::MnxDocumentGlobalMeasuresItemRepeatStart
    {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: MnxDocumentGlobalMeasuresItemRepeatStart,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemRepeatStart>
        for MnxDocumentGlobalMeasuresItemRepeatStart
    {
        fn from(_value: super::MnxDocumentGlobalMeasuresItemRepeatStart) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemSegno {
        class:
            ::std::result::Result<::std::option::Option<super::StyleClass>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        glyph:
            ::std::result::Result<::std::option::Option<super::SmuflGlyph>, ::std::string::String>,
        location: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemSegno {
        fn default() -> Self {
            Self {
                class: Ok(Default::default()),
                color: Ok(Default::default()),
                glyph: Ok(Default::default()),
                location: Err("no value supplied for location".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemSegno {
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StyleClass>>,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn glyph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflGlyph>>,
            T::Error: ::std::fmt::Display,
        {
            self.glyph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for glyph: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemSegno>
        for super::MnxDocumentGlobalMeasuresItemSegno
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemSegno,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                class: value.class?,
                color: value.color?,
                glyph: value.glyph?,
                location: value.location?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemSegno>
        for MnxDocumentGlobalMeasuresItemSegno
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemSegno) -> Self {
            Self {
                class: Ok(value.class),
                color: Ok(value.color),
                glyph: Ok(value.glyph),
                location: Ok(value.location),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemTemposItem {
        bpm: ::std::result::Result<i64, ::std::string::String>,
        location: ::std::result::Result<
            ::std::option::Option<super::RhythmicPosition>,
            ::std::string::String,
        >,
        value: ::std::result::Result<super::NoteValue, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemTemposItem {
        fn default() -> Self {
            Self {
                bpm: Err("no value supplied for bpm".to_string()),
                location: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemTemposItem {
        pub fn bpm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bpm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bpm: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RhythmicPosition>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemTemposItem>
        for super::MnxDocumentGlobalMeasuresItemTemposItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemTemposItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bpm: value.bpm?,
                location: value.location?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemTemposItem>
        for MnxDocumentGlobalMeasuresItemTemposItem
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemTemposItem) -> Self {
            Self {
                bpm: Ok(value.bpm),
                location: Ok(value.location),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalMeasuresItemTime {
        count: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
        unit: ::std::result::Result<
            super::MnxDocumentGlobalMeasuresItemTimeUnit,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentGlobalMeasuresItemTime {
        fn default() -> Self {
            Self {
                count: Err("no value supplied for count".to_string()),
                unit: Err("no value supplied for unit".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalMeasuresItemTime {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {}", e));
            self
        }
        pub fn unit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentGlobalMeasuresItemTimeUnit>,
            T::Error: ::std::fmt::Display,
        {
            self.unit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unit: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalMeasuresItemTime>
        for super::MnxDocumentGlobalMeasuresItemTime
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalMeasuresItemTime,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
                unit: value.unit?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalMeasuresItemTime>
        for MnxDocumentGlobalMeasuresItemTime
    {
        fn from(value: super::MnxDocumentGlobalMeasuresItemTime) -> Self {
            Self {
                count: Ok(value.count),
                unit: Ok(value.unit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentGlobalStylesItem {
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        selector: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentGlobalStylesItem {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                selector: Err("no value supplied for selector".to_string()),
            }
        }
    }
    impl MnxDocumentGlobalStylesItem {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn selector<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.selector = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for selector: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentGlobalStylesItem> for super::MnxDocumentGlobalStylesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentGlobalStylesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                selector: value.selector?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentGlobalStylesItem> for MnxDocumentGlobalStylesItem {
        fn from(value: super::MnxDocumentGlobalStylesItem) -> Self {
            Self {
                color: Ok(value.color),
                selector: Ok(value.selector),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentLayoutsItem {
        content: ::std::result::Result<super::SystemLayoutContent, ::std::string::String>,
        id: ::std::result::Result<super::Id, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentLayoutsItem {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl MnxDocumentLayoutsItem {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SystemLayoutContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentLayoutsItem> for super::MnxDocumentLayoutsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentLayoutsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentLayoutsItem> for MnxDocumentLayoutsItem {
        fn from(value: super::MnxDocumentLayoutsItem) -> Self {
            Self {
                content: Ok(value.content),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentMnx {
        support: ::std::result::Result<
            ::std::option::Option<super::MnxDocumentMnxSupport>,
            ::std::string::String,
        >,
        version: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentMnx {
        fn default() -> Self {
            Self {
                support: Ok(Default::default()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl MnxDocumentMnx {
        pub fn support<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MnxDocumentMnxSupport>>,
            T::Error: ::std::fmt::Display,
        {
            self.support = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for support: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentMnx> for super::MnxDocumentMnx {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentMnx,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                support: value.support?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentMnx> for MnxDocumentMnx {
        fn from(value: super::MnxDocumentMnx) -> Self {
            Self {
                support: Ok(value.support),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentMnxSupport {
        use_accidental_display:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentMnxSupport {
        fn default() -> Self {
            Self {
                use_accidental_display: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentMnxSupport {
        pub fn use_accidental_display<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.use_accidental_display = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for use_accidental_display: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentMnxSupport> for super::MnxDocumentMnxSupport {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentMnxSupport,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                use_accidental_display: value.use_accidental_display?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentMnxSupport> for MnxDocumentMnxSupport {
        fn from(value: super::MnxDocumentMnxSupport) -> Self {
            Self {
                use_accidental_display: Ok(value.use_accidental_display),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentPartsItem {
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        measures: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItem>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        smufl_font:
            ::std::result::Result<::std::option::Option<super::SmuflFont>, ::std::string::String>,
        staves: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentPartsItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                measures: Ok(Default::default()),
                name: Ok(Default::default()),
                short_name: Ok(Default::default()),
                smufl_font: Ok(Default::default()),
                staves: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentPartsItem {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn measures<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.measures = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measures: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {}", e));
            self
        }
        pub fn smufl_font<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflFont>>,
            T::Error: ::std::fmt::Display,
        {
            self.smufl_font = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for smufl_font: {}", e));
            self
        }
        pub fn staves<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.staves = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staves: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentPartsItem> for super::MnxDocumentPartsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentPartsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                measures: value.measures?,
                name: value.name?,
                short_name: value.short_name?,
                smufl_font: value.smufl_font?,
                staves: value.staves?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentPartsItem> for MnxDocumentPartsItem {
        fn from(value: super::MnxDocumentPartsItem) -> Self {
            Self {
                id: Ok(value.id),
                measures: Ok(value.measures),
                name: Ok(value.name),
                short_name: Ok(value.short_name),
                smufl_font: Ok(value.smufl_font),
                staves: Ok(value.staves),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentPartsItemMeasuresItem {
        beams: ::std::result::Result<::std::option::Option<super::BeamList>, ::std::string::String>,
        clefs: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemClefsItem>,
            ::std::string::String,
        >,
        dynamics: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemDynamicsItem>,
            ::std::string::String,
        >,
        ottavas: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemOttavasItem>,
            ::std::string::String,
        >,
        sequences: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemSequencesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentPartsItemMeasuresItem {
        fn default() -> Self {
            Self {
                beams: Ok(Default::default()),
                clefs: Ok(Default::default()),
                dynamics: Ok(Default::default()),
                ottavas: Ok(Default::default()),
                sequences: Err("no value supplied for sequences".to_string()),
            }
        }
    }
    impl MnxDocumentPartsItemMeasuresItem {
        pub fn beams<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BeamList>>,
            T::Error: ::std::fmt::Display,
        {
            self.beams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for beams: {}", e));
            self
        }
        pub fn clefs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemClefsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.clefs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clefs: {}", e));
            self
        }
        pub fn dynamics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemDynamicsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.dynamics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dynamics: {}", e));
            self
        }
        pub fn ottavas<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemOttavasItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ottavas = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ottavas: {}", e));
            self
        }
        pub fn sequences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentPartsItemMeasuresItemSequencesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sequences = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sequences: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentPartsItemMeasuresItem>
        for super::MnxDocumentPartsItemMeasuresItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentPartsItemMeasuresItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                beams: value.beams?,
                clefs: value.clefs?,
                dynamics: value.dynamics?,
                ottavas: value.ottavas?,
                sequences: value.sequences?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentPartsItemMeasuresItem>
        for MnxDocumentPartsItemMeasuresItem
    {
        fn from(value: super::MnxDocumentPartsItemMeasuresItem) -> Self {
            Self {
                beams: Ok(value.beams),
                clefs: Ok(value.clefs),
                dynamics: Ok(value.dynamics),
                ottavas: Ok(value.ottavas),
                sequences: Ok(value.sequences),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentPartsItemMeasuresItemClefsItem {
        clef: ::std::result::Result<
            super::MnxDocumentPartsItemMeasuresItemClefsItemClef,
            ::std::string::String,
        >,
        position: ::std::result::Result<
            ::std::option::Option<super::RhythmicPosition>,
            ::std::string::String,
        >,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentPartsItemMeasuresItemClefsItem {
        fn default() -> Self {
            Self {
                clef: Err("no value supplied for clef".to_string()),
                position: Ok(Default::default()),
                staff: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentPartsItemMeasuresItemClefsItem {
        pub fn clef<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentPartsItemMeasuresItemClefsItemClef>,
            T::Error: ::std::fmt::Display,
        {
            self.clef = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clef: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RhythmicPosition>>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentPartsItemMeasuresItemClefsItem>
        for super::MnxDocumentPartsItemMeasuresItemClefsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentPartsItemMeasuresItemClefsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clef: value.clef?,
                position: value.position?,
                staff: value.staff?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentPartsItemMeasuresItemClefsItem>
        for MnxDocumentPartsItemMeasuresItemClefsItem
    {
        fn from(value: super::MnxDocumentPartsItemMeasuresItemClefsItem) -> Self {
            Self {
                clef: Ok(value.clef),
                position: Ok(value.position),
                staff: Ok(value.staff),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentPartsItemMeasuresItemClefsItemClef {
        class:
            ::std::result::Result<::std::option::Option<super::StyleClass>, ::std::string::String>,
        color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        glyph:
            ::std::result::Result<::std::option::Option<super::SmuflGlyph>, ::std::string::String>,
        octave: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        sign: ::std::result::Result<
            super::MnxDocumentPartsItemMeasuresItemClefsItemClefSign,
            ::std::string::String,
        >,
        staff_position: ::std::result::Result<super::StaffPosition, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentPartsItemMeasuresItemClefsItemClef {
        fn default() -> Self {
            Self {
                class: Ok(Default::default()),
                color: Ok(Default::default()),
                glyph: Ok(Default::default()),
                octave: Ok(Default::default()),
                sign: Err("no value supplied for sign".to_string()),
                staff_position: Err("no value supplied for staff_position".to_string()),
            }
        }
    }
    impl MnxDocumentPartsItemMeasuresItemClefsItemClef {
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StyleClass>>,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn glyph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflGlyph>>,
            T::Error: ::std::fmt::Display,
        {
            self.glyph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for glyph: {}", e));
            self
        }
        pub fn octave<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.octave = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for octave: {}", e));
            self
        }
        pub fn sign<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentPartsItemMeasuresItemClefsItemClefSign>,
            T::Error: ::std::fmt::Display,
        {
            self.sign = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sign: {}", e));
            self
        }
        pub fn staff_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaffPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_position: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentPartsItemMeasuresItemClefsItemClef>
        for super::MnxDocumentPartsItemMeasuresItemClefsItemClef
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentPartsItemMeasuresItemClefsItemClef,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                class: value.class?,
                color: value.color?,
                glyph: value.glyph?,
                octave: value.octave?,
                sign: value.sign?,
                staff_position: value.staff_position?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentPartsItemMeasuresItemClefsItemClef>
        for MnxDocumentPartsItemMeasuresItemClefsItemClef
    {
        fn from(value: super::MnxDocumentPartsItemMeasuresItemClefsItemClef) -> Self {
            Self {
                class: Ok(value.class),
                color: Ok(value.color),
                glyph: Ok(value.glyph),
                octave: Ok(value.octave),
                sign: Ok(value.sign),
                staff_position: Ok(value.staff_position),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentPartsItemMeasuresItemDynamicsItem {
        glyph:
            ::std::result::Result<::std::option::Option<super::SmuflGlyph>, ::std::string::String>,
        position: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentPartsItemMeasuresItemDynamicsItem {
        fn default() -> Self {
            Self {
                glyph: Ok(Default::default()),
                position: Err("no value supplied for position".to_string()),
                staff: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
                voice: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentPartsItemMeasuresItemDynamicsItem {
        pub fn glyph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflGlyph>>,
            T::Error: ::std::fmt::Display,
        {
            self.glyph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for glyph: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentPartsItemMeasuresItemDynamicsItem>
        for super::MnxDocumentPartsItemMeasuresItemDynamicsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentPartsItemMeasuresItemDynamicsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                glyph: value.glyph?,
                position: value.position?,
                staff: value.staff?,
                value: value.value?,
                voice: value.voice?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentPartsItemMeasuresItemDynamicsItem>
        for MnxDocumentPartsItemMeasuresItemDynamicsItem
    {
        fn from(value: super::MnxDocumentPartsItemMeasuresItemDynamicsItem) -> Self {
            Self {
                glyph: Ok(value.glyph),
                position: Ok(value.position),
                staff: Ok(value.staff),
                value: Ok(value.value),
                voice: Ok(value.voice),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentPartsItemMeasuresItemOttavasItem {
        end: ::std::result::Result<super::MeasureRhythmicPosition, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        position: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        value: ::std::result::Result<
            super::MnxDocumentPartsItemMeasuresItemOttavasItemValue,
            ::std::string::String,
        >,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentPartsItemMeasuresItemOttavasItem {
        fn default() -> Self {
            Self {
                end: Err("no value supplied for end".to_string()),
                orient: Ok(Default::default()),
                position: Err("no value supplied for position".to_string()),
                staff: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
                voice: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentPartsItemMeasuresItemOttavasItem {
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureRhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {}", e));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MnxDocumentPartsItemMeasuresItemOttavasItemValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentPartsItemMeasuresItemOttavasItem>
        for super::MnxDocumentPartsItemMeasuresItemOttavasItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentPartsItemMeasuresItemOttavasItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end: value.end?,
                orient: value.orient?,
                position: value.position?,
                staff: value.staff?,
                value: value.value?,
                voice: value.voice?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentPartsItemMeasuresItemOttavasItem>
        for MnxDocumentPartsItemMeasuresItemOttavasItem
    {
        fn from(value: super::MnxDocumentPartsItemMeasuresItemOttavasItem) -> Self {
            Self {
                end: Ok(value.end),
                orient: Ok(value.orient),
                position: Ok(value.position),
                staff: Ok(value.staff),
                value: Ok(value.value),
                voice: Ok(value.voice),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentPartsItemMeasuresItemSequencesItem {
        content: ::std::result::Result<super::SequenceContent, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentPartsItemMeasuresItemSequencesItem {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                orient: Ok(Default::default()),
                staff: Ok(Default::default()),
                voice: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentPartsItemMeasuresItemSequencesItem {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentPartsItemMeasuresItemSequencesItem>
        for super::MnxDocumentPartsItemMeasuresItemSequencesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentPartsItemMeasuresItemSequencesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                orient: value.orient?,
                staff: value.staff?,
                voice: value.voice?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentPartsItemMeasuresItemSequencesItem>
        for MnxDocumentPartsItemMeasuresItemSequencesItem
    {
        fn from(value: super::MnxDocumentPartsItemMeasuresItemSequencesItem) -> Self {
            Self {
                content: Ok(value.content),
                orient: Ok(value.orient),
                staff: Ok(value.staff),
                voice: Ok(value.voice),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentScoresItem {
        layout: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        multimeasure_rests: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentScoresItemMultimeasureRestsItem>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        pages: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentScoresItemPagesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentScoresItem {
        fn default() -> Self {
            Self {
                layout: Ok(Default::default()),
                multimeasure_rests: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                pages: Ok(Default::default()),
            }
        }
    }
    impl MnxDocumentScoresItem {
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {}", e));
            self
        }
        pub fn multimeasure_rests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentScoresItemMultimeasureRestsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.multimeasure_rests = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for multimeasure_rests: {}",
                    e
                )
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn pages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MnxDocumentScoresItemPagesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.pages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pages: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentScoresItem> for super::MnxDocumentScoresItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentScoresItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                layout: value.layout?,
                multimeasure_rests: value.multimeasure_rests?,
                name: value.name?,
                pages: value.pages?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentScoresItem> for MnxDocumentScoresItem {
        fn from(value: super::MnxDocumentScoresItem) -> Self {
            Self {
                layout: Ok(value.layout),
                multimeasure_rests: Ok(value.multimeasure_rests),
                name: Ok(value.name),
                pages: Ok(value.pages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentScoresItemMultimeasureRestsItem {
        duration: ::std::result::Result<i64, ::std::string::String>,
        label: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        start: ::std::result::Result<super::MeasureNumber, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentScoresItemMultimeasureRestsItem {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                label: Ok(Default::default()),
                start: Err("no value supplied for start".to_string()),
            }
        }
    }
    impl MnxDocumentScoresItemMultimeasureRestsItem {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentScoresItemMultimeasureRestsItem>
        for super::MnxDocumentScoresItemMultimeasureRestsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentScoresItemMultimeasureRestsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                label: value.label?,
                start: value.start?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentScoresItemMultimeasureRestsItem>
        for MnxDocumentScoresItemMultimeasureRestsItem
    {
        fn from(value: super::MnxDocumentScoresItemMultimeasureRestsItem) -> Self {
            Self {
                duration: Ok(value.duration),
                label: Ok(value.label),
                start: Ok(value.start),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentScoresItemPagesItem {
        layout: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        systems: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentScoresItemPagesItemSystemsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MnxDocumentScoresItemPagesItem {
        fn default() -> Self {
            Self {
                layout: Ok(Default::default()),
                systems: Err("no value supplied for systems".to_string()),
            }
        }
    }
    impl MnxDocumentScoresItemPagesItem {
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {}", e));
            self
        }
        pub fn systems<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentScoresItemPagesItemSystemsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.systems = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for systems: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentScoresItemPagesItem>
        for super::MnxDocumentScoresItemPagesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentScoresItemPagesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                layout: value.layout?,
                systems: value.systems?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentScoresItemPagesItem>
        for MnxDocumentScoresItemPagesItem
    {
        fn from(value: super::MnxDocumentScoresItemPagesItem) -> Self {
            Self {
                layout: Ok(value.layout),
                systems: Ok(value.systems),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentScoresItemPagesItemSystemsItem {
        layout: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        layout_changes: ::std::result::Result<
            ::std::vec::Vec<super::MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem>,
            ::std::string::String,
        >,
        measure: ::std::result::Result<super::MeasureNumber, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentScoresItemPagesItemSystemsItem {
        fn default() -> Self {
            Self {
                layout: Ok(Default::default()),
                layout_changes: Ok(Default::default()),
                measure: Err("no value supplied for measure".to_string()),
            }
        }
    }
    impl MnxDocumentScoresItemPagesItemSystemsItem {
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {}", e));
            self
        }
        pub fn layout_changes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.layout_changes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout_changes: {}", e));
            self
        }
        pub fn measure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.measure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measure: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentScoresItemPagesItemSystemsItem>
        for super::MnxDocumentScoresItemPagesItemSystemsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentScoresItemPagesItemSystemsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                layout: value.layout?,
                layout_changes: value.layout_changes?,
                measure: value.measure?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentScoresItemPagesItemSystemsItem>
        for MnxDocumentScoresItemPagesItemSystemsItem
    {
        fn from(value: super::MnxDocumentScoresItemPagesItemSystemsItem) -> Self {
            Self {
                layout: Ok(value.layout),
                layout_changes: Ok(value.layout_changes),
                measure: Ok(value.measure),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem {
        layout: ::std::result::Result<super::Id, ::std::string::String>,
        location: ::std::result::Result<super::MeasureRhythmicPosition, ::std::string::String>,
    }
    impl ::std::default::Default for MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem {
        fn default() -> Self {
            Self {
                layout: Err("no value supplied for layout".to_string()),
                location: Err("no value supplied for location".to_string()),
            }
        }
    }
    impl MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem {
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureRhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem>
        for super::MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                layout: value.layout?,
                location: value.location?,
            })
        }
    }
    impl ::std::convert::From<super::MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem>
        for MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem
    {
        fn from(value: super::MnxDocumentScoresItemPagesItemSystemsItemLayoutChangesItem) -> Self {
            Self {
                layout: Ok(value.layout),
                location: Ok(value.location),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NoteValue {
        base: ::std::result::Result<super::NoteValueBase, ::std::string::String>,
        dots: ::std::result::Result<
            ::std::option::Option<super::PositiveInteger>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NoteValue {
        fn default() -> Self {
            Self {
                base: Err("no value supplied for base".to_string()),
                dots: Ok(Default::default()),
            }
        }
    }
    impl NoteValue {
        pub fn base<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueBase>,
            T::Error: ::std::fmt::Display,
        {
            self.base = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base: {}", e));
            self
        }
        pub fn dots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveInteger>>,
            T::Error: ::std::fmt::Display,
        {
            self.dots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dots: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NoteValue> for super::NoteValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NoteValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base: value.base?,
                dots: value.dots?,
            })
        }
    }
    impl ::std::convert::From<super::NoteValue> for NoteValue {
        fn from(value: super::NoteValue) -> Self {
            Self {
                base: Ok(value.base),
                dots: Ok(value.dots),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NoteValueQuantity {
        duration: ::std::result::Result<super::NoteValue, ::std::string::String>,
        multiple: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
    }
    impl ::std::default::Default for NoteValueQuantity {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                multiple: Err("no value supplied for multiple".to_string()),
            }
        }
    }
    impl NoteValueQuantity {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValue>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn multiple<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.multiple = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for multiple: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NoteValueQuantity> for super::NoteValueQuantity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NoteValueQuantity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                multiple: value.multiple?,
            })
        }
    }
    impl ::std::convert::From<super::NoteValueQuantity> for NoteValueQuantity {
        fn from(value: super::NoteValueQuantity) -> Self {
            Self {
                duration: Ok(value.duration),
                multiple: Ok(value.multiple),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RhythmicPosition {
        fraction: ::std::result::Result<super::Fraction, ::std::string::String>,
        grace_index: ::std::result::Result<
            ::std::option::Option<super::IntegerUnsigned>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RhythmicPosition {
        fn default() -> Self {
            Self {
                fraction: Err("no value supplied for fraction".to_string()),
                grace_index: Ok(Default::default()),
            }
        }
    }
    impl RhythmicPosition {
        pub fn fraction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Fraction>,
            T::Error: ::std::fmt::Display,
        {
            self.fraction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fraction: {}", e));
            self
        }
        pub fn grace_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IntegerUnsigned>>,
            T::Error: ::std::fmt::Display,
        {
            self.grace_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grace_index: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<RhythmicPosition> for super::RhythmicPosition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RhythmicPosition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fraction: value.fraction?,
                grace_index: value.grace_index?,
            })
        }
    }
    impl ::std::convert::From<super::RhythmicPosition> for RhythmicPosition {
        fn from(value: super::RhythmicPosition) -> Self {
            Self {
                fraction: Ok(value.fraction),
                grace_index: Ok(value.grace_index),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SequenceContentItem {
        subtype_0:
            ::std::result::Result<::std::option::Option<super::Event>, ::std::string::String>,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::SequenceContentItemSubtype1>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::SequenceContentItemSubtype2>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::SequenceContentItemSubtype3>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SequenceContentItem {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
            }
        }
    }
    impl SequenceContentItem {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Event>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SequenceContentItemSubtype1>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SequenceContentItemSubtype2>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SequenceContentItemSubtype3>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SequenceContentItem> for super::SequenceContentItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SequenceContentItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
            })
        }
    }
    impl ::std::convert::From<super::SequenceContentItem> for SequenceContentItem {
        fn from(value: super::SequenceContentItem) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SequenceContentItemSubtype1 {
        class:
            ::std::result::Result<::std::option::Option<super::StyleClass>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        content: ::std::result::Result<::std::vec::Vec<super::Event>, ::std::string::String>,
        grace_type: ::std::result::Result<
            ::std::option::Option<super::SequenceContentItemSubtype1GraceType>,
            ::std::string::String,
        >,
        slash: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SequenceContentItemSubtype1 {
        fn default() -> Self {
            Self {
                class: Ok(Default::default()),
                color: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                grace_type: Ok(Default::default()),
                slash: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SequenceContentItemSubtype1 {
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StyleClass>>,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Event>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn grace_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SequenceContentItemSubtype1GraceType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.grace_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grace_type: {}", e));
            self
        }
        pub fn slash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.slash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for slash: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SequenceContentItemSubtype1> for super::SequenceContentItemSubtype1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SequenceContentItemSubtype1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                class: value.class?,
                color: value.color?,
                content: value.content?,
                grace_type: value.grace_type?,
                slash: value.slash?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SequenceContentItemSubtype1> for SequenceContentItemSubtype1 {
        fn from(value: super::SequenceContentItemSubtype1) -> Self {
            Self {
                class: Ok(value.class),
                color: Ok(value.color),
                content: Ok(value.content),
                grace_type: Ok(value.grace_type),
                slash: Ok(value.slash),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SequenceContentItemSubtype2 {
        bracket: ::std::result::Result<
            ::std::option::Option<super::SequenceContentItemSubtype2Bracket>,
            ::std::string::String,
        >,
        content: ::std::result::Result<super::SequenceContent, ::std::string::String>,
        inner: ::std::result::Result<super::NoteValueQuantity, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        outer: ::std::result::Result<super::NoteValueQuantity, ::std::string::String>,
        show_number: ::std::result::Result<
            ::std::option::Option<super::TupletDisplaySetting>,
            ::std::string::String,
        >,
        show_value: ::std::result::Result<
            ::std::option::Option<super::TupletDisplaySetting>,
            ::std::string::String,
        >,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SequenceContentItemSubtype2 {
        fn default() -> Self {
            Self {
                bracket: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                inner: Err("no value supplied for inner".to_string()),
                orient: Ok(Default::default()),
                outer: Err("no value supplied for outer".to_string()),
                show_number: Ok(Default::default()),
                show_value: Ok(Default::default()),
                staff: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SequenceContentItemSubtype2 {
        pub fn bracket<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SequenceContentItemSubtype2Bracket>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bracket = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bracket: {}", e));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn inner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueQuantity>,
            T::Error: ::std::fmt::Display,
        {
            self.inner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inner: {}", e));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {}", e));
            self
        }
        pub fn outer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueQuantity>,
            T::Error: ::std::fmt::Display,
        {
            self.outer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outer: {}", e));
            self
        }
        pub fn show_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TupletDisplaySetting>>,
            T::Error: ::std::fmt::Display,
        {
            self.show_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show_number: {}", e));
            self
        }
        pub fn show_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TupletDisplaySetting>>,
            T::Error: ::std::fmt::Display,
        {
            self.show_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show_value: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SequenceContentItemSubtype2> for super::SequenceContentItemSubtype2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SequenceContentItemSubtype2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bracket: value.bracket?,
                content: value.content?,
                inner: value.inner?,
                orient: value.orient?,
                outer: value.outer?,
                show_number: value.show_number?,
                show_value: value.show_value?,
                staff: value.staff?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SequenceContentItemSubtype2> for SequenceContentItemSubtype2 {
        fn from(value: super::SequenceContentItemSubtype2) -> Self {
            Self {
                bracket: Ok(value.bracket),
                content: Ok(value.content),
                inner: Ok(value.inner),
                orient: Ok(value.orient),
                outer: Ok(value.outer),
                show_number: Ok(value.show_number),
                show_value: Ok(value.show_value),
                staff: Ok(value.staff),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SequenceContentItemSubtype3 {
        duration: ::std::result::Result<super::Fraction, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SequenceContentItemSubtype3 {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SequenceContentItemSubtype3 {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Fraction>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SequenceContentItemSubtype3> for super::SequenceContentItemSubtype3 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SequenceContentItemSubtype3,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SequenceContentItemSubtype3> for SequenceContentItemSubtype3 {
        fn from(value: super::SequenceContentItemSubtype3) -> Self {
            Self {
                duration: Ok(value.duration),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SystemLayoutContentItemSourcesItem {
        label:
            ::std::result::Result<::std::option::Option<super::StaffLabel>, ::std::string::String>,
        labelref: ::std::result::Result<
            ::std::option::Option<super::StaffLabelref>,
            ::std::string::String,
        >,
        part: ::std::result::Result<super::Id, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        stem: ::std::result::Result<
            ::std::option::Option<super::StemDirection>,
            ::std::string::String,
        >,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
    }
    impl ::std::default::Default for SystemLayoutContentItemSourcesItem {
        fn default() -> Self {
            Self {
                label: Ok(Default::default()),
                labelref: Ok(Default::default()),
                part: Err("no value supplied for part".to_string()),
                staff: Ok(Default::default()),
                stem: Ok(Default::default()),
                voice: Ok(Default::default()),
            }
        }
    }
    impl SystemLayoutContentItemSourcesItem {
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffLabel>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn labelref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffLabelref>>,
            T::Error: ::std::fmt::Display,
        {
            self.labelref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for labelref: {}", e));
            self
        }
        pub fn part<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.part = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for part: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
        pub fn stem<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StemDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.stem = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stem: {}", e));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SystemLayoutContentItemSourcesItem>
        for super::SystemLayoutContentItemSourcesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SystemLayoutContentItemSourcesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                labelref: value.labelref?,
                part: value.part?,
                staff: value.staff?,
                stem: value.stem?,
                voice: value.voice?,
            })
        }
    }
    impl ::std::convert::From<super::SystemLayoutContentItemSourcesItem>
        for SystemLayoutContentItemSourcesItem
    {
        fn from(value: super::SystemLayoutContentItemSourcesItem) -> Self {
            Self {
                label: Ok(value.label),
                labelref: Ok(value.labelref),
                part: Ok(value.part),
                staff: Ok(value.staff),
                stem: Ok(value.stem),
                voice: Ok(value.voice),
            }
        }
    }
}
