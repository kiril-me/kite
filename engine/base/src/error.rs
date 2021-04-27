// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.


//! Kite error types
use thiserror::Error;

use arrow::error::ArrowError;
use datafusion::error::DataFusionError;

//use std::error;
//use std::fmt::{Display, Formatter};
use std::io;
use std::result;

// Result type for operations that could result in an [KiteError]
pub type Result<T> = result::Result<T, KiteError>;


#[derive(Debug, Error)]
pub enum KiteError {
    #[error("Generic meta store error")]
    GenericError,

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error("Arrow error")]
    Arrow(#[from] ArrowError),

    #[error("DataFusion error")]
    DataFusion(#[from] DataFusionError),
}
