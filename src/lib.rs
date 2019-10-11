// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License Author                                                   |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@metatomic.io>                                                         |
// +-----------------------------------------------------------------------------------------------+

#![crate_name = "videocore"]

// -------------------------------------------------------------------------------------------------
// LINKING
// -------------------------------------------------------------------------------------------------

#[link(name = "bcm_host")]
#[link(name = "vcos")]
extern {}

#[cfg(target_arch = "arm")]
#[link(name = "vchiq_arm")]
extern {}

// -------------------------------------------------------------------------------------------------
// MODULES
// -------------------------------------------------------------------------------------------------

pub mod bcm_host;
pub mod display;
pub mod dispmanx;
pub mod image;
pub mod vchi;
