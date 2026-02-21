use crate::v4::CellMetadata;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "output_type")]
pub enum Output {
    #[serde(rename = "stream")]
    Stream {
        #[serde(default)]
        name: Option<String>,
        #[serde(rename = "stream", default)]
        stream: Option<String>,
        #[serde(default)]
        text: Vec<String>,
    },
    #[serde(rename = "pyout")]
    PyOut {
        #[serde(default)]
        prompt_number: Option<i32>,
        #[serde(default)]
        text: Vec<String>,
        #[serde(default)]
        html: Option<Vec<String>>,
        #[serde(default)]
        metadata: Value,
    },
    #[serde(rename = "display_data")]
    DisplayData {
        #[serde(default)]
        data: Value,
        #[serde(default)]
        metadata: Value,
    },
    #[serde(rename = "pyerr")]
    PyErr {
        #[serde(default)]
        ename: Option<String>,
        #[serde(default)]
        evalue: Option<String>,
        #[serde(default)]
        traceback: Vec<String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Notebook {
    #[serde(default)]
    pub metadata: Option<Value>,
    pub nbformat: i32,
    #[serde(default)]
    pub nbformat_minor: Option<i32>,
    #[serde(default)]
    pub worksheets: Option<Vec<Worksheet>>,
}

#[derive(Deserialize, Debug)]
pub struct Worksheet {
    #[serde(default)]
    pub cells: Vec<Cell>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "heading")]
    Heading {
        level: i32,
        metadata: CellMetadata,
        #[serde(default)]
        source: Vec<String>,
    },
    #[serde(rename = "markdown")]
    Markdown {
        metadata: CellMetadata,
        #[serde(default)]
        source: Vec<String>,
        #[serde(default)]
        attachments: Option<Value>,
    },
    #[serde(rename = "code")]
    Code {
        metadata: CellMetadata,
        #[serde(default)]
        prompt_number: Option<i32>,
        #[serde(default)]
        input: Option<Vec<String>>,
        #[serde(default)]
        language: Option<String>,
        #[serde(default)]
        outputs: Vec<Output>,
    },
    #[serde(rename = "raw")]
    Raw {
        metadata: CellMetadata,
        #[serde(default)]
        source: Vec<String>,
    },
}
