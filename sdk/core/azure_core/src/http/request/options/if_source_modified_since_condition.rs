// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    date,
    http::headers::{self, Header, HeaderName},
};
use typespec_client_core::date::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSourceModifiedSinceCondition {
    Modified(OffsetDateTime),
    Unmodified(OffsetDateTime),
}

impl Header for IfSourceModifiedSinceCondition {
    fn name(&self) -> HeaderName {
        match self {
            IfSourceModifiedSinceCondition::Modified(_) => headers::SOURCE_IF_MODIFIED_SINCE,
            IfSourceModifiedSinceCondition::Unmodified(_) => headers::SOURCE_IF_UNMODIFIED_SINCE,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfSourceModifiedSinceCondition::Modified(date)
            | IfSourceModifiedSinceCondition::Unmodified(date) => date::to_rfc7231(date).into(),
        }
    }
}
