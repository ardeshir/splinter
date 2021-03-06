// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use crate::actix_web::HttpResponse;
use crate::futures::IntoFuture;
use crate::protocol;
use crate::rest_api::{
    ErrorResponse, HandlerFunction, Method, ProtocolVersionRangeGuard, Resource,
};

#[cfg(feature = "biome-profile")]
use crate::biome::profile::store::{UserProfileStore, UserProfileStoreError};

#[cfg(feature = "authorization")]
use crate::biome::rest_api::BIOME_USER_READ_PERMISSION;

pub fn make_profiles_routes(profile_store: Arc<dyn UserProfileStore>) -> Resource {
    let resource =
        Resource::build("/biome/profiles/{id}").add_request_guard(ProtocolVersionRangeGuard::new(
            protocol::BIOME_FETCH_PROFILES_PROTOCOL_MIN,
            protocol::BIOME_PROTOCOL_VERSION,
        ));
    #[cfg(feature = "authorization")]
    {
        resource.add_method(
            Method::Get,
            BIOME_USER_READ_PERMISSION,
            add_fetch_profile_method(profile_store.clone()),
        )
    }
    #[cfg(not(feature = "authorization"))]
    {
        resource.add_method(Method::Get, add_fetch_profile_method(profile_store.clone()))
    }
}

fn add_fetch_profile_method(profile_store: Arc<dyn UserProfileStore>) -> HandlerFunction {
    Box::new(move |request, _| {
        let profile_store = profile_store.clone();
        let user_id = if let Some(t) = request.match_info().get("id") {
            t.to_string()
        } else {
            return Box::new(
                HttpResponse::BadRequest()
                    .json(ErrorResponse::bad_request(
                        &"Failed to process request: no user id".to_string(),
                    ))
                    .into_future(),
            );
        };
        Box::new(match profile_store.get_profile(&user_id) {
            Ok(profile) => HttpResponse::Ok().json(profile).into_future(),
            Err(err) => {
                debug!("Failed to get profile from the database {}", err);
                match err {
                    UserProfileStoreError::InvalidArgument(_) => HttpResponse::NotFound()
                        .json(ErrorResponse::not_found(&format!(
                            "User ID not found: {}",
                            &user_id
                        )))
                        .into_future(),
                    _ => HttpResponse::InternalServerError()
                        .json(ErrorResponse::internal_error())
                        .into_future(),
                }
            }
        })
    })
}
