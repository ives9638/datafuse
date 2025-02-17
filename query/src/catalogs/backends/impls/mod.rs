//  Copyright 2021 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

mod embedded_backend;
mod meta_cached;
mod meta_remote;
mod meta_sync;
mod remote_backend;

pub use embedded_backend::MetaEmbeddedSync;
pub use meta_cached::MetaCached;
pub use meta_remote::MetaRemote;
pub use meta_sync::MetaSync;
pub use remote_backend::MetaRemoteSync;
