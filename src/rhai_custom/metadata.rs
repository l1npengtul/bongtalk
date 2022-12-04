use crate::error::{BResult, BongTalkError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
struct RhaiFnDef {
    #[serde(rename = "baseHash")]
    pub base_hash: i64,
    #[serde(rename = "fullHash")]
    pub full_hash: i64,
    pub namespace: String,
    pub access: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "numParams")]
    pub num_params: i64,
    pub params: Vec<RhaiParams>,
    #[serde(rename = "returnType")]
    pub return_type: String,
    pub signature: String,
    #[serde(rename = "docComments")]
    pub doc_comments: Option<Vec<String>>,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RhaiModule {
    pub modules: HashMap<String, RhaiModule, ahash::RandomState>,
    pub functions: Vec<RhaiFnDef>,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RhaiParams {
    pub params: Vec<(Option<String>, String)>,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RhaiMetadata {
    pub modules: HashMap<String, RhaiModule, ahash::RandomState>,
    pub functions: Vec<RhaiFnDef>,
}

pub fn generate_rhaifn_metadata(data: &str) -> BResult<RhaiMetadata> {
    serde_json::from_str(data).map_err(|why| BongTalkError::EngineInit(why.into()))
}

// test json
// {
//     "modules": {
//       "general_kenobi": {
//         "modules": {
//           "thesusutest": {
//             "functions": [
//               {
//                 "baseHash": 15650973899437713901,
//                 "fullHash": 17853253219818374055,
//                 "namespace": "internal",
//                 "access": "public ",
//                 "name": "hi",
//                 "type": "native",
//                 "numParams": 1,
//                 "params": [
//                   {
//                     "name": "a",
//                     "type": "i64"
//                   }
//                 ],
//                 "returnType": "String",
//                 "signature": "hi(a: i64) -> String"
//               }
//             ]
//           }
//         },
//         "functions": [
//           {
//             "baseHash": 12408987324883194094,
//             "fullHash": 9423086063473556132,
//             "namespace": "internal",
//             "access": "public",
//             "name": "hello_there",
//             "type": "native",
//             "numParams": 1,
//             "params": [
//               {
//                 "name": "n",
//                 "type": "i64"
//               }
//             ],
//             "returnType": "String",
//             "signature": "hello_there(n: i64) -> String",
//             "docComments": [
//               "/// Returns a string where \"hello there\" is repeated `n` times."
//             ]
//           }
//         ]
//       }
//     },
//     "functions": [
//       {
//         "baseHash": 14367592870419963779,
//         "fullHash": 13958874329462201363,
//         "namespace": "global",
//         "access": "public",
//         "name": "minus",
//         "type": "native",
//         "numParams": 2,
//         "params": [
//           {
//             "type": "i64"
//           },
//           {
//             "type": "i64"
//           }
//         ],
//         "returnType": "i64",
//         "signature": "minus(_: i64, _: i64) -> i64"
//       }
//     ]
//   }
