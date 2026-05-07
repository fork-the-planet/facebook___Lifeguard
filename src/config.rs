/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::pyrefly::sys_info::SysInfo;
use crate::traits::SysInfoExt;

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub sys_info: SysInfo,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            sys_info: SysInfo::lg_default(),
        }
    }
}

impl AnalysisConfig {
    pub fn new(sys_info: SysInfo) -> Self {
        Self { sys_info }
    }
}
