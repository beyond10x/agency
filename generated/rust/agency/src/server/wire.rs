// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`
//! Every generated declaration, as JSON, in the renderings the published wire contracts fix.
//!
//! Generated from the model beside the types it crosses, so a field renamed in the specification
//! is renamed here in the same regeneration. An absent optional field is omitted rather
//! than sent as `null`, which is what the `required` list of the published schema says.

use crate::server::json;

/// Writes `agency.agents.Agent.State` as JSON.
pub fn encode_agency_agents_agent_state(value: &crate::agents::AgentState, out: &mut String) {
    match value {
        crate::agents::AgentState::Provisioned => json::push_text(out, "Provisioned"),
        crate::agents::AgentState::Retired => json::push_text(out, "Retired"),
    }
}

/// Reads `agency.agents.Agent.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_agents_agent_state(value: &json::Value, at: &str) -> Result<crate::agents::AgentState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Provisioned`, `Retired`")? {
        "Provisioned" => crate::agents::AgentState::Provisioned,
        "Retired" => crate::agents::AgentState::Retired,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Provisioned`, `Retired`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.agents.AgentDefinition.State` as JSON.
pub fn encode_agency_agents_agent_definition_state(value: &crate::agents::AgentDefinitionState, out: &mut String) {
    match value {
        crate::agents::AgentDefinitionState::Checked => json::push_text(out, "Checked"),
        crate::agents::AgentDefinitionState::Deprecated => json::push_text(out, "Deprecated"),
        crate::agents::AgentDefinitionState::Draft => json::push_text(out, "Draft"),
        crate::agents::AgentDefinitionState::Published => json::push_text(out, "Published"),
    }
}

/// Reads `agency.agents.AgentDefinition.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_agents_agent_definition_state(value: &json::Value, at: &str) -> Result<crate::agents::AgentDefinitionState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Checked`, `Deprecated`, `Draft`, `Published`")? {
        "Checked" => crate::agents::AgentDefinitionState::Checked,
        "Deprecated" => crate::agents::AgentDefinitionState::Deprecated,
        "Draft" => crate::agents::AgentDefinitionState::Draft,
        "Published" => crate::agents::AgentDefinitionState::Published,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Checked`, `Deprecated`, `Draft`, `Published`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.agents.AgentId` as JSON.
pub fn encode_agency_agents_agent_id(value: &crate::agents::AgentId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.agents.AgentId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_agents_agent_id(value: &json::Value, at: &str) -> Result<crate::agents::AgentId, json::DecodeError> {
    Ok(crate::agents::AgentId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.agents.AgentSpecDocument` as JSON.
pub fn encode_agency_agents_agent_spec_document(value: &crate::agents::AgentSpecDocument, out: &mut String) {
    json::push_value(out, &value.0);
}

/// Reads `agency.agents.AgentSpecDocument` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_agents_agent_spec_document(value: &json::Value, at: &str) -> Result<crate::agents::AgentSpecDocument, json::DecodeError> {
    Ok(crate::agents::AgentSpecDocument(json::value_at(value, at)))
}

/// Writes `agency.agents.CheckVerdict` as JSON.
pub fn encode_agency_agents_check_verdict(value: &crate::agents::CheckVerdict, out: &mut String) {
    match value {
        crate::agents::CheckVerdict::Accepted => json::push_text(out, "Accepted"),
        crate::agents::CheckVerdict::Refused => json::push_text(out, "Refused"),
    }
}

/// Reads `agency.agents.CheckVerdict` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_agents_check_verdict(value: &json::Value, at: &str) -> Result<crate::agents::CheckVerdict, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Accepted`, `Refused`")? {
        "Accepted" => crate::agents::CheckVerdict::Accepted,
        "Refused" => crate::agents::CheckVerdict::Refused,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Accepted`, `Refused`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.agents.DefinitionId` as JSON.
pub fn encode_agency_agents_definition_id(value: &crate::agents::DefinitionId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.agents.DefinitionId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_agents_definition_id(value: &json::Value, at: &str) -> Result<crate::agents::DefinitionId, json::DecodeError> {
    Ok(crate::agents::DefinitionId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.auth.ConnectorsCustody` as JSON.
pub fn encode_agency_auth_connectors_custody(value: &crate::auth::ConnectorsCustody, out: &mut String) {
    out.push('{');
    json::member(out, "instance_id");
    json::push_text(out, &value.instance_id);
    out.push('}');
}

/// Reads `agency.auth.ConnectorsCustody` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_connectors_custody(value: &json::Value, at: &str) -> Result<crate::auth::ConnectorsCustody, json::DecodeError> {
    Ok(crate::auth::ConnectorsCustody {
        instance_id: {
            let at0 = json::nested(at, "instance_id");
            let member0 = json::member_at(value, at, "instance_id")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.auth.Credential.State` as JSON.
pub fn encode_agency_auth_credential_state(value: &crate::auth::CredentialState, out: &mut String) {
    match value {
        crate::auth::CredentialState::Revoked => json::push_text(out, "Revoked"),
        crate::auth::CredentialState::Stored => json::push_text(out, "Stored"),
    }
}

/// Reads `agency.auth.Credential.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_credential_state(value: &json::Value, at: &str) -> Result<crate::auth::CredentialState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Revoked`, `Stored`")? {
        "Revoked" => crate::auth::CredentialState::Revoked,
        "Stored" => crate::auth::CredentialState::Stored,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Revoked`, `Stored`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.CredentialGrant.State` as JSON.
pub fn encode_agency_auth_credential_grant_state(value: &crate::auth::CredentialGrantState, out: &mut String) {
    match value {
        crate::auth::CredentialGrantState::Granted => json::push_text(out, "Granted"),
        crate::auth::CredentialGrantState::Revoked => json::push_text(out, "Revoked"),
    }
}

/// Reads `agency.auth.CredentialGrant.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_credential_grant_state(value: &json::Value, at: &str) -> Result<crate::auth::CredentialGrantState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Granted`, `Revoked`")? {
        "Granted" => crate::auth::CredentialGrantState::Granted,
        "Revoked" => crate::auth::CredentialGrantState::Revoked,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Granted`, `Revoked`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.CredentialId` as JSON.
pub fn encode_agency_auth_credential_id(value: &crate::auth::CredentialId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.auth.CredentialId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_credential_id(value: &json::Value, at: &str) -> Result<crate::auth::CredentialId, json::DecodeError> {
    Ok(crate::auth::CredentialId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.auth.CredentialKind` as JSON.
pub fn encode_agency_auth_credential_kind(value: &crate::auth::CredentialKind, out: &mut String) {
    match value {
        crate::auth::CredentialKind::OperatorLogin => json::push_text(out, "OperatorLogin"),
        crate::auth::CredentialKind::ApiKey => json::push_text(out, "ApiKey"),
        crate::auth::CredentialKind::OAuthToken => json::push_text(out, "OAuthToken"),
        crate::auth::CredentialKind::ConnectorInstance => json::push_text(out, "ConnectorInstance"),
    }
}

/// Reads `agency.auth.CredentialKind` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_credential_kind(value: &json::Value, at: &str) -> Result<crate::auth::CredentialKind, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `OperatorLogin`, `ApiKey`, `OAuthToken`, `ConnectorInstance`")? {
        "OperatorLogin" => crate::auth::CredentialKind::OperatorLogin,
        "ApiKey" => crate::auth::CredentialKind::ApiKey,
        "OAuthToken" => crate::auth::CredentialKind::OAuthToken,
        "ConnectorInstance" => crate::auth::CredentialKind::ConnectorInstance,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `OperatorLogin`, `ApiKey`, `OAuthToken`, `ConnectorInstance`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.CredentialScope` as JSON.
pub fn encode_agency_auth_credential_scope(value: &crate::auth::CredentialScope, out: &mut String) {
    out.push('{');
    json::member(out, "models");
    out.push('[');
    for (index0, item0) in value.models.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "providers");
    out.push('[');
    for (index0, item0) in value.providers.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "tools");
    out.push('[');
    for (index0, item0) in value.tools.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_tools_tool_id(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agency.auth.CredentialScope` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_credential_scope(value: &json::Value, at: &str) -> Result<crate::auth::CredentialScope, json::DecodeError> {
    Ok(crate::auth::CredentialScope {
        models: {
            let at0 = json::nested(at, "models");
            let member0 = json::member_at(value, at, "models")?;
            {
                let mut items0 = Vec::new();
                for (index0, element0) in json::items_at(member0, &at0, "an array")?.iter().enumerate() {
                    let nested0 = json::nested(&at0, &index0.to_string());
                    items0.push(json::text_at(element0, &nested0, "a string")?.to_owned());
                }
                items0
            }
        },
        providers: {
            let at1 = json::nested(at, "providers");
            let member1 = json::member_at(value, at, "providers")?;
            {
                let mut items1 = Vec::new();
                for (index1, element1) in json::items_at(member1, &at1, "an array")?.iter().enumerate() {
                    let nested1 = json::nested(&at1, &index1.to_string());
                    items1.push(json::text_at(element1, &nested1, "a string")?.to_owned());
                }
                items1
            }
        },
        tools: {
            let at2 = json::nested(at, "tools");
            let member2 = json::member_at(value, at, "tools")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agency_tools_tool_id(element2, &nested2)?);
                }
                items2
            }
        },
    })
}

/// Writes `agency.auth.Custody` as JSON.
pub fn encode_agency_auth_custody(value: &crate::auth::Custody, out: &mut String) {
    match value {
        crate::auth::Custody::AgencyVault => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "AgencyVault");
            out.push('}');
        }
        crate::auth::Custody::ConnectorsInstance(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "ConnectorsInstance");
            json::member(out, "value");
            encode_agency_auth_connectors_custody(&*held, out);
            out.push('}');
        }
        crate::auth::Custody::HostFile(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "HostFile");
            json::member(out, "value");
            encode_agency_auth_host_file_custody(&*held, out);
            out.push('}');
        }
    }
}

/// Reads `agency.auth.Custody` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_custody(value: &json::Value, at: &str) -> Result<crate::auth::Custody, json::DecodeError> {
    let tag = json::member_at(value, at, "kind")?;
    let at_tag = json::nested(at, "kind");
    Ok(match json::text_at(tag, &at_tag, "one of `AgencyVault`, `ConnectorsInstance`, `HostFile`")? {
        "AgencyVault" => {
            if let Some(found) = value.member("value") {
                return Err(json::DecodeError::of(&json::nested(at, "value"), "no `value`: `AgencyVault` carries nothing", found));
            }
            crate::auth::Custody::AgencyVault
        }
        "ConnectorsInstance" => crate::auth::Custody::ConnectorsInstance({
            let at1 = json::nested(at, "value");
            let member1 = json::member_at(value, at, "value")?;
            decode_agency_auth_connectors_custody(member1, &at1)?
        }),
        "HostFile" => crate::auth::Custody::HostFile({
            let at2 = json::nested(at, "value");
            let member2 = json::member_at(value, at, "value")?;
            decode_agency_auth_host_file_custody(member2, &at2)?
        }),
        other => return Err(json::DecodeError { at: at_tag.clone(), expected: "one of `AgencyVault`, `ConnectorsInstance`, `HostFile`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.DecisionMode` as JSON.
pub fn encode_agency_auth_decision_mode(value: &crate::auth::DecisionMode, out: &mut String) {
    match value {
        crate::auth::DecisionMode::Frame => json::push_text(out, "Frame"),
        crate::auth::DecisionMode::Ask => json::push_text(out, "Ask"),
        crate::auth::DecisionMode::Observe => json::push_text(out, "Observe"),
    }
}

/// Reads `agency.auth.DecisionMode` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_decision_mode(value: &json::Value, at: &str) -> Result<crate::auth::DecisionMode, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Frame`, `Ask`, `Observe`")? {
        "Frame" => crate::auth::DecisionMode::Frame,
        "Ask" => crate::auth::DecisionMode::Ask,
        "Observe" => crate::auth::DecisionMode::Observe,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Frame`, `Ask`, `Observe`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.DefinitionSubject` as JSON.
pub fn encode_agency_auth_definition_subject(value: &crate::auth::DefinitionSubject, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    out.push('}');
}

/// Reads `agency.auth.DefinitionSubject` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_definition_subject(value: &json::Value, at: &str) -> Result<crate::auth::DefinitionSubject, json::DecodeError> {
    Ok(crate::auth::DefinitionSubject {
        definition_id: {
            let at0 = json::nested(at, "definition_id");
            let member0 = json::member_at(value, at, "definition_id")?;
            decode_agency_agents_definition_id(member0, &at0)?
        },
    })
}

/// Writes `agency.auth.GrantId` as JSON.
pub fn encode_agency_auth_grant_id(value: &crate::auth::GrantId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.auth.GrantId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_grant_id(value: &json::Value, at: &str) -> Result<crate::auth::GrantId, json::DecodeError> {
    Ok(crate::auth::GrantId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.auth.GrantPurpose` as JSON.
pub fn encode_agency_auth_grant_purpose(value: &crate::auth::GrantPurpose, out: &mut String) {
    match value {
        crate::auth::GrantPurpose::ModelCalls => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "ModelCalls");
            out.push('}');
        }
        crate::auth::GrantPurpose::ToolCalls(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "ToolCalls");
            json::member(out, "value");
            encode_agency_auth_tool_calls_purpose(&*held, out);
            out.push('}');
        }
    }
}

/// Reads `agency.auth.GrantPurpose` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_grant_purpose(value: &json::Value, at: &str) -> Result<crate::auth::GrantPurpose, json::DecodeError> {
    let tag = json::member_at(value, at, "kind")?;
    let at_tag = json::nested(at, "kind");
    Ok(match json::text_at(tag, &at_tag, "one of `ModelCalls`, `ToolCalls`")? {
        "ModelCalls" => {
            if let Some(found) = value.member("value") {
                return Err(json::DecodeError::of(&json::nested(at, "value"), "no `value`: `ModelCalls` carries nothing", found));
            }
            crate::auth::GrantPurpose::ModelCalls
        }
        "ToolCalls" => crate::auth::GrantPurpose::ToolCalls({
            let at1 = json::nested(at, "value");
            let member1 = json::member_at(value, at, "value")?;
            decode_agency_auth_tool_calls_purpose(member1, &at1)?
        }),
        other => return Err(json::DecodeError { at: at_tag.clone(), expected: "one of `ModelCalls`, `ToolCalls`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.HostFileCustody` as JSON.
pub fn encode_agency_auth_host_file_custody(value: &crate::auth::HostFileCustody, out: &mut String) {
    out.push('{');
    json::member(out, "path");
    json::push_text(out, &value.path);
    out.push('}');
}

/// Reads `agency.auth.HostFileCustody` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_host_file_custody(value: &json::Value, at: &str) -> Result<crate::auth::HostFileCustody, json::DecodeError> {
    Ok(crate::auth::HostFileCustody {
        path: {
            let at0 = json::nested(at, "path");
            let member0 = json::member_at(value, at, "path")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.auth.Policy.State` as JSON.
pub fn encode_agency_auth_policy_state(value: &crate::auth::PolicyState, out: &mut String) {
    match value {
        crate::auth::PolicyState::Active => json::push_text(out, "Active"),
        crate::auth::PolicyState::Draft => json::push_text(out, "Draft"),
        crate::auth::PolicyState::Superseded => json::push_text(out, "Superseded"),
    }
}

/// Reads `agency.auth.Policy.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_policy_state(value: &json::Value, at: &str) -> Result<crate::auth::PolicyState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Active`, `Draft`, `Superseded`")? {
        "Active" => crate::auth::PolicyState::Active,
        "Draft" => crate::auth::PolicyState::Draft,
        "Superseded" => crate::auth::PolicyState::Superseded,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Active`, `Draft`, `Superseded`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.PolicyId` as JSON.
pub fn encode_agency_auth_policy_id(value: &crate::auth::PolicyId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.auth.PolicyId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_policy_id(value: &json::Value, at: &str) -> Result<crate::auth::PolicyId, json::DecodeError> {
    Ok(crate::auth::PolicyId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.auth.Principal.State` as JSON.
pub fn encode_agency_auth_principal_state(value: &crate::auth::PrincipalState, out: &mut String) {
    match value {
        crate::auth::PrincipalState::Disabled => json::push_text(out, "Disabled"),
        crate::auth::PrincipalState::Invited => json::push_text(out, "Invited"),
    }
}

/// Reads `agency.auth.Principal.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_principal_state(value: &json::Value, at: &str) -> Result<crate::auth::PrincipalState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Disabled`, `Invited`")? {
        "Disabled" => crate::auth::PrincipalState::Disabled,
        "Invited" => crate::auth::PrincipalState::Invited,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Disabled`, `Invited`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.PrincipalId` as JSON.
pub fn encode_agency_auth_principal_id(value: &crate::auth::PrincipalId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.auth.PrincipalId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_principal_id(value: &json::Value, at: &str) -> Result<crate::auth::PrincipalId, json::DecodeError> {
    Ok(crate::auth::PrincipalId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.auth.PrincipalKind` as JSON.
pub fn encode_agency_auth_principal_kind(value: &crate::auth::PrincipalKind, out: &mut String) {
    match value {
        crate::auth::PrincipalKind::Human => json::push_text(out, "Human"),
        crate::auth::PrincipalKind::Agent => json::push_text(out, "Agent"),
        crate::auth::PrincipalKind::Service => json::push_text(out, "Service"),
    }
}

/// Reads `agency.auth.PrincipalKind` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_principal_kind(value: &json::Value, at: &str) -> Result<crate::auth::PrincipalKind, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Human`, `Agent`, `Service`")? {
        "Human" => crate::auth::PrincipalKind::Human,
        "Agent" => crate::auth::PrincipalKind::Agent,
        "Service" => crate::auth::PrincipalKind::Service,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Human`, `Agent`, `Service`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.RiskCeiling` as JSON.
pub fn encode_agency_auth_risk_ceiling(value: &crate::auth::RiskCeiling, out: &mut String) {
    match value {
        crate::auth::RiskCeiling::Low => json::push_text(out, "Low"),
        crate::auth::RiskCeiling::Medium => json::push_text(out, "Medium"),
        crate::auth::RiskCeiling::High => json::push_text(out, "High"),
    }
}

/// Reads `agency.auth.RiskCeiling` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_risk_ceiling(value: &json::Value, at: &str) -> Result<crate::auth::RiskCeiling, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Low`, `Medium`, `High`")? {
        "Low" => crate::auth::RiskCeiling::Low,
        "Medium" => crate::auth::RiskCeiling::Medium,
        "High" => crate::auth::RiskCeiling::High,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Low`, `Medium`, `High`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.Role` as JSON.
pub fn encode_agency_auth_role(value: &crate::auth::Role, out: &mut String) {
    match value {
        crate::auth::Role::Operator => json::push_text(out, "Operator"),
        crate::auth::Role::Supervisor => json::push_text(out, "Supervisor"),
        crate::auth::Role::Viewer => json::push_text(out, "Viewer"),
        crate::auth::Role::Agent => json::push_text(out, "Agent"),
    }
}

/// Reads `agency.auth.Role` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_role(value: &json::Value, at: &str) -> Result<crate::auth::Role, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Operator`, `Supervisor`, `Viewer`, `Agent`")? {
        "Operator" => crate::auth::Role::Operator,
        "Supervisor" => crate::auth::Role::Supervisor,
        "Viewer" => crate::auth::Role::Viewer,
        "Agent" => crate::auth::Role::Agent,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Operator`, `Supervisor`, `Viewer`, `Agent`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.Rule` as JSON.
pub fn encode_agency_auth_rule(value: &crate::auth::Rule, out: &mut String) {
    out.push('{');
    json::member(out, "subject");
    encode_agency_auth_rule_subject(&value.subject, out);
    if let Some(held0) = &value.tool_id {
        json::member(out, "tool_id");
        encode_agency_tools_tool_id(&*held0, out);
    }
    if let Some(held0) = &value.verbs {
        json::member(out, "verbs");
        out.push('[');
        for (index1, item1) in (*held0).iter().enumerate() {
            if index1 > 0 {
                out.push(',');
            }
            encode_agency_shared_verb(&*item1, out);
        }
        out.push(']');
    }
    json::member(out, "risk_ceiling");
    encode_agency_auth_risk_ceiling(&value.risk_ceiling, out);
    json::member(out, "decision_mode");
    encode_agency_auth_decision_mode(&value.decision_mode, out);
    json::member(out, "budget_ceiling");
    encode_agency_shared_run_budget(&value.budget_ceiling, out);
    json::member(out, "hermetic");
    json::push_bool(out, value.hermetic);
    out.push('}');
}

/// Reads `agency.auth.Rule` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_rule(value: &json::Value, at: &str) -> Result<crate::auth::Rule, json::DecodeError> {
    Ok(crate::auth::Rule {
        subject: {
            let at0 = json::nested(at, "subject");
            let member0 = json::member_at(value, at, "subject")?;
            decode_agency_auth_rule_subject(member0, &at0)?
        },
        tool_id: match value.member("tool_id") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "tool_id");
                Some(decode_agency_tools_tool_id(member1, &at1)?)
            }
        },
        verbs: match value.member("verbs") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "verbs");
                Some({
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agency_shared_verb(element2, &nested2)?);
                }
                items2
            })
            }
        },
        risk_ceiling: {
            let at3 = json::nested(at, "risk_ceiling");
            let member3 = json::member_at(value, at, "risk_ceiling")?;
            decode_agency_auth_risk_ceiling(member3, &at3)?
        },
        decision_mode: {
            let at4 = json::nested(at, "decision_mode");
            let member4 = json::member_at(value, at, "decision_mode")?;
            decode_agency_auth_decision_mode(member4, &at4)?
        },
        budget_ceiling: {
            let at5 = json::nested(at, "budget_ceiling");
            let member5 = json::member_at(value, at, "budget_ceiling")?;
            decode_agency_shared_run_budget(member5, &at5)?
        },
        hermetic: {
            let at6 = json::nested(at, "hermetic");
            let member6 = json::member_at(value, at, "hermetic")?;
            json::bool_at(member6, &at6, "a boolean")?
        },
    })
}

/// Writes `agency.auth.RuleSubject` as JSON.
pub fn encode_agency_auth_rule_subject(value: &crate::auth::RuleSubject, out: &mut String) {
    match value {
        crate::auth::RuleSubject::Definition(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Definition");
            json::member(out, "value");
            encode_agency_auth_definition_subject(&*held, out);
            out.push('}');
        }
        crate::auth::RuleSubject::Team(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Team");
            json::member(out, "value");
            encode_agency_auth_team_subject(&*held, out);
            out.push('}');
        }
        crate::auth::RuleSubject::Tenant(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Tenant");
            json::member(out, "value");
            encode_agency_auth_tenant_subject(&*held, out);
            out.push('}');
        }
        crate::auth::RuleSubject::Workforce(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Workforce");
            json::member(out, "value");
            encode_agency_auth_workforce_subject(&*held, out);
            out.push('}');
        }
    }
}

/// Reads `agency.auth.RuleSubject` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_rule_subject(value: &json::Value, at: &str) -> Result<crate::auth::RuleSubject, json::DecodeError> {
    let tag = json::member_at(value, at, "kind")?;
    let at_tag = json::nested(at, "kind");
    Ok(match json::text_at(tag, &at_tag, "one of `Definition`, `Team`, `Tenant`, `Workforce`")? {
        "Definition" => crate::auth::RuleSubject::Definition({
            let at0 = json::nested(at, "value");
            let member0 = json::member_at(value, at, "value")?;
            decode_agency_auth_definition_subject(member0, &at0)?
        }),
        "Team" => crate::auth::RuleSubject::Team({
            let at1 = json::nested(at, "value");
            let member1 = json::member_at(value, at, "value")?;
            decode_agency_auth_team_subject(member1, &at1)?
        }),
        "Tenant" => crate::auth::RuleSubject::Tenant({
            let at2 = json::nested(at, "value");
            let member2 = json::member_at(value, at, "value")?;
            decode_agency_auth_tenant_subject(member2, &at2)?
        }),
        "Workforce" => crate::auth::RuleSubject::Workforce({
            let at3 = json::nested(at, "value");
            let member3 = json::member_at(value, at, "value")?;
            decode_agency_auth_workforce_subject(member3, &at3)?
        }),
        other => return Err(json::DecodeError { at: at_tag.clone(), expected: "one of `Definition`, `Team`, `Tenant`, `Workforce`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.TeamSubject` as JSON.
pub fn encode_agency_auth_team_subject(value: &crate::auth::TeamSubject, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    out.push('}');
}

/// Reads `agency.auth.TeamSubject` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_team_subject(value: &json::Value, at: &str) -> Result<crate::auth::TeamSubject, json::DecodeError> {
    Ok(crate::auth::TeamSubject {
        team_id: {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
    })
}

/// Writes `agency.auth.Tenant.State` as JSON.
pub fn encode_agency_auth_tenant_state(value: &crate::auth::TenantState, out: &mut String) {
    match value {
        crate::auth::TenantState::Created => json::push_text(out, "Created"),
    }
}

/// Reads `agency.auth.Tenant.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_tenant_state(value: &json::Value, at: &str) -> Result<crate::auth::TenantState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Created`")? {
        "Created" => crate::auth::TenantState::Created,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Created`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.auth.TenantId` as JSON.
pub fn encode_agency_auth_tenant_id(value: &crate::auth::TenantId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.auth.TenantId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_tenant_id(value: &json::Value, at: &str) -> Result<crate::auth::TenantId, json::DecodeError> {
    Ok(crate::auth::TenantId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.auth.TenantSubject` as JSON.
pub fn encode_agency_auth_tenant_subject(value: &crate::auth::TenantSubject, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    out.push('}');
}

/// Reads `agency.auth.TenantSubject` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_tenant_subject(value: &json::Value, at: &str) -> Result<crate::auth::TenantSubject, json::DecodeError> {
    Ok(crate::auth::TenantSubject {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
    })
}

/// Writes `agency.auth.ToolCallsPurpose` as JSON.
pub fn encode_agency_auth_tool_calls_purpose(value: &crate::auth::ToolCallsPurpose, out: &mut String) {
    out.push('{');
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    out.push('}');
}

/// Reads `agency.auth.ToolCallsPurpose` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_tool_calls_purpose(value: &json::Value, at: &str) -> Result<crate::auth::ToolCallsPurpose, json::DecodeError> {
    Ok(crate::auth::ToolCallsPurpose {
        tool_id: {
            let at0 = json::nested(at, "tool_id");
            let member0 = json::member_at(value, at, "tool_id")?;
            decode_agency_tools_tool_id(member0, &at0)?
        },
    })
}

/// Writes `agency.auth.WorkforceSubject` as JSON.
pub fn encode_agency_auth_workforce_subject(value: &crate::auth::WorkforceSubject, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    out.push('}');
}

/// Reads `agency.auth.WorkforceSubject` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_auth_workforce_subject(value: &json::Value, at: &str) -> Result<crate::auth::WorkforceSubject, json::DecodeError> {
    Ok(crate::auth::WorkforceSubject {
        workforce_id: {
            let at0 = json::nested(at, "workforce_id");
            let member0 = json::member_at(value, at, "workforce_id")?;
            decode_agency_teams_workforce_id(member0, &at0)?
        },
    })
}

/// Writes `agency.runs.ApprovalId` as JSON.
pub fn encode_agency_runs_approval_id(value: &crate::runs::ApprovalId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.runs.ApprovalId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_approval_id(value: &json::Value, at: &str) -> Result<crate::runs::ApprovalId, json::DecodeError> {
    Ok(crate::runs::ApprovalId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.runs.CredentialSource` as JSON.
pub fn encode_agency_runs_credential_source(value: &crate::runs::CredentialSource, out: &mut String) {
    match value {
        crate::runs::CredentialSource::OperatorLogin => json::push_text(out, "OperatorLogin"),
        crate::runs::CredentialSource::ApiKey => json::push_text(out, "ApiKey"),
        crate::runs::CredentialSource::Loopback => json::push_text(out, "Loopback"),
        crate::runs::CredentialSource::None => json::push_text(out, "None"),
    }
}

/// Reads `agency.runs.CredentialSource` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_credential_source(value: &json::Value, at: &str) -> Result<crate::runs::CredentialSource, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `OperatorLogin`, `ApiKey`, `Loopback`, `None`")? {
        "OperatorLogin" => crate::runs::CredentialSource::OperatorLogin,
        "ApiKey" => crate::runs::CredentialSource::ApiKey,
        "Loopback" => crate::runs::CredentialSource::Loopback,
        "None" => crate::runs::CredentialSource::None,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `OperatorLogin`, `ApiKey`, `Loopback`, `None`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.runs.LoomSessionId` as JSON.
pub fn encode_agency_runs_loom_session_id(value: &crate::runs::LoomSessionId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.runs.LoomSessionId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_loom_session_id(value: &json::Value, at: &str) -> Result<crate::runs::LoomSessionId, json::DecodeError> {
    Ok(crate::runs::LoomSessionId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.runs.Run.State` as JSON.
pub fn encode_agency_runs_run_state(value: &crate::runs::RunState, out: &mut String) {
    match value {
        crate::runs::RunState::Admitted => json::push_text(out, "Admitted"),
        crate::runs::RunState::Failed => json::push_text(out, "Failed"),
        crate::runs::RunState::Proposed => json::push_text(out, "Proposed"),
        crate::runs::RunState::Requested => json::push_text(out, "Requested"),
        crate::runs::RunState::Running => json::push_text(out, "Running"),
        crate::runs::RunState::Stopped => json::push_text(out, "Stopped"),
    }
}

/// Reads `agency.runs.Run.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_run_state(value: &json::Value, at: &str) -> Result<crate::runs::RunState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Admitted`, `Failed`, `Proposed`, `Requested`, `Running`, `Stopped`")? {
        "Admitted" => crate::runs::RunState::Admitted,
        "Failed" => crate::runs::RunState::Failed,
        "Proposed" => crate::runs::RunState::Proposed,
        "Requested" => crate::runs::RunState::Requested,
        "Running" => crate::runs::RunState::Running,
        "Stopped" => crate::runs::RunState::Stopped,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Admitted`, `Failed`, `Proposed`, `Requested`, `Running`, `Stopped`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.runs.RunApproval.State` as JSON.
pub fn encode_agency_runs_run_approval_state(value: &crate::runs::RunApprovalState, out: &mut String) {
    match value {
        crate::runs::RunApprovalState::Denied => json::push_text(out, "Denied"),
        crate::runs::RunApprovalState::Granted => json::push_text(out, "Granted"),
        crate::runs::RunApprovalState::Pending => json::push_text(out, "Pending"),
    }
}

/// Reads `agency.runs.RunApproval.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_run_approval_state(value: &json::Value, at: &str) -> Result<crate::runs::RunApprovalState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Denied`, `Granted`, `Pending`")? {
        "Denied" => crate::runs::RunApprovalState::Denied,
        "Granted" => crate::runs::RunApprovalState::Granted,
        "Pending" => crate::runs::RunApprovalState::Pending,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Denied`, `Granted`, `Pending`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.runs.RunEndReason` as JSON.
pub fn encode_agency_runs_run_end_reason(value: &crate::runs::RunEndReason, out: &mut String) {
    out.push('{');
    json::member(out, "reason");
    json::push_text(out, &value.reason);
    out.push('}');
}

/// Reads `agency.runs.RunEndReason` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_run_end_reason(value: &json::Value, at: &str) -> Result<crate::runs::RunEndReason, json::DecodeError> {
    Ok(crate::runs::RunEndReason {
        reason: {
            let at0 = json::nested(at, "reason");
            let member0 = json::member_at(value, at, "reason")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.runs.RunId` as JSON.
pub fn encode_agency_runs_run_id(value: &crate::runs::RunId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.runs.RunId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_run_id(value: &json::Value, at: &str) -> Result<crate::runs::RunId, json::DecodeError> {
    Ok(crate::runs::RunId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.runs.RunOutcome` as JSON.
pub fn encode_agency_runs_run_outcome(value: &crate::runs::RunOutcome, out: &mut String) {
    match value {
        crate::runs::RunOutcome::Failed(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Failed");
            json::member(out, "value");
            encode_agency_runs_run_end_reason(&*held, out);
            out.push('}');
        }
        crate::runs::RunOutcome::Proposal => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Proposal");
            out.push('}');
        }
        crate::runs::RunOutcome::Stopped(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Stopped");
            json::member(out, "value");
            encode_agency_runs_run_end_reason(&*held, out);
            out.push('}');
        }
    }
}

/// Reads `agency.runs.RunOutcome` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_run_outcome(value: &json::Value, at: &str) -> Result<crate::runs::RunOutcome, json::DecodeError> {
    let tag = json::member_at(value, at, "kind")?;
    let at_tag = json::nested(at, "kind");
    Ok(match json::text_at(tag, &at_tag, "one of `Failed`, `Proposal`, `Stopped`")? {
        "Failed" => crate::runs::RunOutcome::Failed({
            let at0 = json::nested(at, "value");
            let member0 = json::member_at(value, at, "value")?;
            decode_agency_runs_run_end_reason(member0, &at0)?
        }),
        "Proposal" => {
            if let Some(found) = value.member("value") {
                return Err(json::DecodeError::of(&json::nested(at, "value"), "no `value`: `Proposal` carries nothing", found));
            }
            crate::runs::RunOutcome::Proposal
        }
        "Stopped" => crate::runs::RunOutcome::Stopped({
            let at2 = json::nested(at, "value");
            let member2 = json::member_at(value, at, "value")?;
            decode_agency_runs_run_end_reason(member2, &at2)?
        }),
        other => return Err(json::DecodeError { at: at_tag.clone(), expected: "one of `Failed`, `Proposal`, `Stopped`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.runs.RunUsage` as JSON.
pub fn encode_agency_runs_run_usage(value: &crate::runs::RunUsage, out: &mut String) {
    out.push('{');
    json::member(out, "model_calls");
    json::push_integer(out, value.model_calls);
    json::member(out, "input_tokens");
    json::push_integer(out, value.input_tokens);
    json::member(out, "output_tokens");
    json::push_integer(out, value.output_tokens);
    json::member(out, "cached_input_tokens");
    json::push_integer(out, value.cached_input_tokens);
    if let Some(held0) = &value.cost_micro_usd {
        json::member(out, "cost_micro_usd");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agency.runs.RunUsage` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_run_usage(value: &json::Value, at: &str) -> Result<crate::runs::RunUsage, json::DecodeError> {
    Ok(crate::runs::RunUsage {
        model_calls: {
            let at0 = json::nested(at, "model_calls");
            let member0 = json::member_at(value, at, "model_calls")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        input_tokens: {
            let at1 = json::nested(at, "input_tokens");
            let member1 = json::member_at(value, at, "input_tokens")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        output_tokens: {
            let at2 = json::nested(at, "output_tokens");
            let member2 = json::member_at(value, at, "output_tokens")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        cached_input_tokens: {
            let at3 = json::nested(at, "cached_input_tokens");
            let member3 = json::member_at(value, at, "cached_input_tokens")?;
            json::integer_at(member3, &at3, "an integer")?
        },
        cost_micro_usd: match value.member("cost_micro_usd") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "cost_micro_usd");
                Some(json::integer_at(member4, &at4, "an integer")?)
            }
        },
    })
}

/// Writes `agency.runs.RunWorkspace` as JSON.
pub fn encode_agency_runs_run_workspace(value: &crate::runs::RunWorkspace, out: &mut String) {
    out.push('{');
    json::member(out, "repository");
    json::push_text(out, &value.repository);
    json::member(out, "base_branch");
    json::push_text(out, &value.base_branch);
    if let Some(held0) = &value.volume_claim {
        json::member(out, "volume_claim");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads `agency.runs.RunWorkspace` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_runs_run_workspace(value: &json::Value, at: &str) -> Result<crate::runs::RunWorkspace, json::DecodeError> {
    Ok(crate::runs::RunWorkspace {
        repository: {
            let at0 = json::nested(at, "repository");
            let member0 = json::member_at(value, at, "repository")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        base_branch: {
            let at1 = json::nested(at, "base_branch");
            let member1 = json::member_at(value, at, "base_branch")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        volume_claim: match value.member("volume_claim") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "volume_claim");
                Some(json::text_at(member2, &at2, "a string")?.to_owned())
            }
        },
    })
}

/// Writes `agency.service.Session.State` as JSON.
pub fn encode_agency_service_session_state(value: &crate::service::SessionState, out: &mut String) {
    match value {
        crate::service::SessionState::Closed => json::push_text(out, "Closed"),
        crate::service::SessionState::Open => json::push_text(out, "Open"),
    }
}

/// Reads `agency.service.Session.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_service_session_state(value: &json::Value, at: &str) -> Result<crate::service::SessionState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Closed`, `Open`")? {
        "Closed" => crate::service::SessionState::Closed,
        "Open" => crate::service::SessionState::Open,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Closed`, `Open`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.service.SessionId` as JSON.
pub fn encode_agency_service_session_id(value: &crate::service::SessionId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.service.SessionId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_service_session_id(value: &json::Value, at: &str) -> Result<crate::service::SessionId, json::DecodeError> {
    Ok(crate::service::SessionId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.service.SessionKind` as JSON.
pub fn encode_agency_service_session_kind(value: &crate::service::SessionKind, out: &mut String) {
    match value {
        crate::service::SessionKind::OperatorCookie => json::push_text(out, "OperatorCookie"),
        crate::service::SessionKind::ApplicationToken => json::push_text(out, "ApplicationToken"),
        crate::service::SessionKind::RunBearer => json::push_text(out, "RunBearer"),
    }
}

/// Reads `agency.service.SessionKind` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_service_session_kind(value: &json::Value, at: &str) -> Result<crate::service::SessionKind, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `OperatorCookie`, `ApplicationToken`, `RunBearer`")? {
        "OperatorCookie" => crate::service::SessionKind::OperatorCookie,
        "ApplicationToken" => crate::service::SessionKind::ApplicationToken,
        "RunBearer" => crate::service::SessionKind::RunBearer,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `OperatorCookie`, `ApplicationToken`, `RunBearer`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.shared.Digest` as JSON.
pub fn encode_agency_shared_digest(value: &crate::shared::Digest, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agency.shared.Digest` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_shared_digest(value: &json::Value, at: &str) -> Result<crate::shared::Digest, json::DecodeError> {
    Ok(crate::shared::Digest(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agency.shared.RunBudget` as JSON.
pub fn encode_agency_shared_run_budget(value: &crate::shared::RunBudget, out: &mut String) {
    out.push('{');
    if let Some(held0) = &value.max_tokens {
        json::member(out, "max_tokens");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.deadline {
        json::member(out, "deadline");
        json::push_text(out, &(*held0).0);
    }
    out.push('}');
}

/// Reads `agency.shared.RunBudget` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_shared_run_budget(value: &json::Value, at: &str) -> Result<crate::shared::RunBudget, json::DecodeError> {
    Ok(crate::shared::RunBudget {
        max_tokens: match value.member("max_tokens") {
            None | Some(json::Value::Null) => None,
            Some(member0) => {
                let at0 = json::nested(at, "max_tokens");
                Some(json::integer_at(member0, &at0, "an integer")?)
            }
        },
        deadline: match value.member("deadline") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "deadline");
                Some(crate::primitives::Timestamp(json::text_at(member1, &at1, "an RFC 3339 instant")?.to_owned()))
            }
        },
    })
}

/// Writes `agency.shared.Verb` as JSON.
pub fn encode_agency_shared_verb(value: &crate::shared::Verb, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agency.shared.Verb` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_shared_verb(value: &json::Value, at: &str) -> Result<crate::shared::Verb, json::DecodeError> {
    Ok(crate::shared::Verb(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agency.teams.Capacity` as JSON.
pub fn encode_agency_teams_capacity(value: &crate::teams::Capacity, out: &mut String) {
    out.push('{');
    json::member(out, "max_concurrent_runs");
    json::push_integer(out, value.max_concurrent_runs);
    if let Some(held0) = &value.max_tokens_per_hour {
        json::member(out, "max_tokens_per_hour");
        json::push_integer(out, *held0);
    }
    out.push('}');
}

/// Reads `agency.teams.Capacity` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_teams_capacity(value: &json::Value, at: &str) -> Result<crate::teams::Capacity, json::DecodeError> {
    Ok(crate::teams::Capacity {
        max_concurrent_runs: {
            let at0 = json::nested(at, "max_concurrent_runs");
            let member0 = json::member_at(value, at, "max_concurrent_runs")?;
            json::integer_at(member0, &at0, "an integer")?
        },
        max_tokens_per_hour: match value.member("max_tokens_per_hour") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "max_tokens_per_hour");
                Some(json::integer_at(member1, &at1, "an integer")?)
            }
        },
    })
}

/// Writes `agency.teams.Swarm.State` as JSON.
pub fn encode_agency_teams_swarm_state(value: &crate::teams::SwarmState, out: &mut String) {
    match value {
        crate::teams::SwarmState::Declared => json::push_text(out, "Declared"),
        crate::teams::SwarmState::Stopped => json::push_text(out, "Stopped"),
    }
}

/// Reads `agency.teams.Swarm.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_teams_swarm_state(value: &json::Value, at: &str) -> Result<crate::teams::SwarmState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Declared`, `Stopped`")? {
        "Declared" => crate::teams::SwarmState::Declared,
        "Stopped" => crate::teams::SwarmState::Stopped,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Declared`, `Stopped`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.teams.SwarmId` as JSON.
pub fn encode_agency_teams_swarm_id(value: &crate::teams::SwarmId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.teams.SwarmId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_teams_swarm_id(value: &json::Value, at: &str) -> Result<crate::teams::SwarmId, json::DecodeError> {
    Ok(crate::teams::SwarmId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.teams.Team.State` as JSON.
pub fn encode_agency_teams_team_state(value: &crate::teams::TeamState, out: &mut String) {
    match value {
        crate::teams::TeamState::Disbanded => json::push_text(out, "Disbanded"),
        crate::teams::TeamState::Formed => json::push_text(out, "Formed"),
    }
}

/// Reads `agency.teams.Team.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_teams_team_state(value: &json::Value, at: &str) -> Result<crate::teams::TeamState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Disbanded`, `Formed`")? {
        "Disbanded" => crate::teams::TeamState::Disbanded,
        "Formed" => crate::teams::TeamState::Formed,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Disbanded`, `Formed`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.teams.TeamId` as JSON.
pub fn encode_agency_teams_team_id(value: &crate::teams::TeamId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.teams.TeamId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_teams_team_id(value: &json::Value, at: &str) -> Result<crate::teams::TeamId, json::DecodeError> {
    Ok(crate::teams::TeamId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.teams.Workforce.State` as JSON.
pub fn encode_agency_teams_workforce_state(value: &crate::teams::WorkforceState, out: &mut String) {
    match value {
        crate::teams::WorkforceState::Defined => json::push_text(out, "Defined"),
        crate::teams::WorkforceState::Retired => json::push_text(out, "Retired"),
        crate::teams::WorkforceState::Staffed => json::push_text(out, "Staffed"),
    }
}

/// Reads `agency.teams.Workforce.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_teams_workforce_state(value: &json::Value, at: &str) -> Result<crate::teams::WorkforceState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Defined`, `Retired`, `Staffed`")? {
        "Defined" => crate::teams::WorkforceState::Defined,
        "Retired" => crate::teams::WorkforceState::Retired,
        "Staffed" => crate::teams::WorkforceState::Staffed,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Defined`, `Retired`, `Staffed`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.teams.WorkforceId` as JSON.
pub fn encode_agency_teams_workforce_id(value: &crate::teams::WorkforceId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.teams.WorkforceId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_teams_workforce_id(value: &json::Value, at: &str) -> Result<crate::teams::WorkforceId, json::DecodeError> {
    Ok(crate::teams::WorkforceId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.tools.BindingId` as JSON.
pub fn encode_agency_tools_binding_id(value: &crate::tools::BindingId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.tools.BindingId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_binding_id(value: &json::Value, at: &str) -> Result<crate::tools::BindingId, json::DecodeError> {
    Ok(crate::tools::BindingId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.tools.BindingScope` as JSON.
pub fn encode_agency_tools_binding_scope(value: &crate::tools::BindingScope, out: &mut String) {
    out.push('{');
    json::member(out, "repositories");
    out.push('[');
    for (index0, item0) in value.repositories.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "paths");
    out.push('[');
    for (index0, item0) in value.paths.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "subjects");
    out.push('[');
    for (index0, item0) in value.subjects.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    out.push('}');
}

/// Reads `agency.tools.BindingScope` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_binding_scope(value: &json::Value, at: &str) -> Result<crate::tools::BindingScope, json::DecodeError> {
    Ok(crate::tools::BindingScope {
        repositories: {
            let at0 = json::nested(at, "repositories");
            let member0 = json::member_at(value, at, "repositories")?;
            {
                let mut items0 = Vec::new();
                for (index0, element0) in json::items_at(member0, &at0, "an array")?.iter().enumerate() {
                    let nested0 = json::nested(&at0, &index0.to_string());
                    items0.push(json::text_at(element0, &nested0, "a string")?.to_owned());
                }
                items0
            }
        },
        paths: {
            let at1 = json::nested(at, "paths");
            let member1 = json::member_at(value, at, "paths")?;
            {
                let mut items1 = Vec::new();
                for (index1, element1) in json::items_at(member1, &at1, "an array")?.iter().enumerate() {
                    let nested1 = json::nested(&at1, &index1.to_string());
                    items1.push(json::text_at(element1, &nested1, "a string")?.to_owned());
                }
                items1
            }
        },
        subjects: {
            let at2 = json::nested(at, "subjects");
            let member2 = json::member_at(value, at, "subjects")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(json::text_at(element2, &nested2, "a string")?.to_owned());
                }
                items2
            }
        },
    })
}

/// Writes `agency.tools.ContractRef` as JSON.
pub fn encode_agency_tools_contract_ref(value: &crate::tools::ContractRef, out: &mut String) {
    out.push('{');
    json::member(out, "toolset");
    json::push_text(out, &value.toolset);
    json::member(out, "version");
    json::push_text(out, &value.version);
    out.push('}');
}

/// Reads `agency.tools.ContractRef` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_contract_ref(value: &json::Value, at: &str) -> Result<crate::tools::ContractRef, json::DecodeError> {
    Ok(crate::tools::ContractRef {
        toolset: {
            let at0 = json::nested(at, "toolset");
            let member0 = json::member_at(value, at, "toolset")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        version: {
            let at1 = json::nested(at, "version");
            let member1 = json::member_at(value, at, "version")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.tools.RiskEnvelope` as JSON.
pub fn encode_agency_tools_risk_envelope(value: &crate::tools::RiskEnvelope, out: &mut String) {
    out.push('{');
    json::member(out, "effects");
    out.push('[');
    for (index0, item0) in value.effects.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        json::push_text(out, &*item0);
    }
    out.push(']');
    json::member(out, "risk");
    encode_agency_tools_tool_risk(&value.risk, out);
    json::member(out, "idempotency");
    json::push_text(out, &value.idempotency);
    out.push('}');
}

/// Reads `agency.tools.RiskEnvelope` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_risk_envelope(value: &json::Value, at: &str) -> Result<crate::tools::RiskEnvelope, json::DecodeError> {
    Ok(crate::tools::RiskEnvelope {
        effects: {
            let at0 = json::nested(at, "effects");
            let member0 = json::member_at(value, at, "effects")?;
            {
                let mut items0 = Vec::new();
                for (index0, element0) in json::items_at(member0, &at0, "an array")?.iter().enumerate() {
                    let nested0 = json::nested(&at0, &index0.to_string());
                    items0.push(json::text_at(element0, &nested0, "a string")?.to_owned());
                }
                items0
            }
        },
        risk: {
            let at1 = json::nested(at, "risk");
            let member1 = json::member_at(value, at, "risk")?;
            decode_agency_tools_tool_risk(member1, &at1)?
        },
        idempotency: {
            let at2 = json::nested(at, "idempotency");
            let member2 = json::member_at(value, at, "idempotency")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.tools.Tool.State` as JSON.
pub fn encode_agency_tools_tool_state(value: &crate::tools::ToolState, out: &mut String) {
    match value {
        crate::tools::ToolState::Registered => json::push_text(out, "Registered"),
        crate::tools::ToolState::Withdrawn => json::push_text(out, "Withdrawn"),
    }
}

/// Reads `agency.tools.Tool.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_tool_state(value: &json::Value, at: &str) -> Result<crate::tools::ToolState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Registered`, `Withdrawn`")? {
        "Registered" => crate::tools::ToolState::Registered,
        "Withdrawn" => crate::tools::ToolState::Withdrawn,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Registered`, `Withdrawn`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.tools.ToolBinding.State` as JSON.
pub fn encode_agency_tools_tool_binding_state(value: &crate::tools::ToolBindingState, out: &mut String) {
    match value {
        crate::tools::ToolBindingState::Granted => json::push_text(out, "Granted"),
        crate::tools::ToolBindingState::Revoked => json::push_text(out, "Revoked"),
    }
}

/// Reads `agency.tools.ToolBinding.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_tool_binding_state(value: &json::Value, at: &str) -> Result<crate::tools::ToolBindingState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Granted`, `Revoked`")? {
        "Granted" => crate::tools::ToolBindingState::Granted,
        "Revoked" => crate::tools::ToolBindingState::Revoked,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Granted`, `Revoked`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.tools.ToolId` as JSON.
pub fn encode_agency_tools_tool_id(value: &crate::tools::ToolId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.tools.ToolId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_tool_id(value: &json::Value, at: &str) -> Result<crate::tools::ToolId, json::DecodeError> {
    Ok(crate::tools::ToolId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.tools.ToolRisk` as JSON.
pub fn encode_agency_tools_tool_risk(value: &crate::tools::ToolRisk, out: &mut String) {
    match value {
        crate::tools::ToolRisk::None => json::push_text(out, "None"),
        crate::tools::ToolRisk::Low => json::push_text(out, "Low"),
        crate::tools::ToolRisk::Medium => json::push_text(out, "Medium"),
        crate::tools::ToolRisk::High => json::push_text(out, "High"),
        crate::tools::ToolRisk::Critical => json::push_text(out, "Critical"),
    }
}

/// Reads `agency.tools.ToolRisk` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_tools_tool_risk(value: &json::Value, at: &str) -> Result<crate::tools::ToolRisk, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `None`, `Low`, `Medium`, `High`, `Critical`")? {
        "None" => crate::tools::ToolRisk::None,
        "Low" => crate::tools::ToolRisk::Low,
        "Medium" => crate::tools::ToolRisk::Medium,
        "High" => crate::tools::ToolRisk::High,
        "Critical" => crate::tools::ToolRisk::Critical,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `None`, `Low`, `Medium`, `High`, `Critical`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.work.AepStoreSource` as JSON.
pub fn encode_agency_work_aep_store_source(value: &crate::work::AepStoreSource, out: &mut String) {
    out.push('{');
    json::member(out, "repo");
    json::push_text(out, &value.repo);
    json::member(out, "filter");
    json::push_text(out, &value.filter);
    out.push('}');
}

/// Reads `agency.work.AepStoreSource` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_aep_store_source(value: &json::Value, at: &str) -> Result<crate::work::AepStoreSource, json::DecodeError> {
    Ok(crate::work::AepStoreSource {
        repo: {
            let at0 = json::nested(at, "repo");
            let member0 = json::member_at(value, at, "repo")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        filter: {
            let at1 = json::nested(at, "filter");
            let member1 = json::member_at(value, at, "filter")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.work.GitHubIssuesSource` as JSON.
pub fn encode_agency_work_git_hub_issues_source(value: &crate::work::GitHubIssuesSource, out: &mut String) {
    out.push('{');
    json::member(out, "repo");
    json::push_text(out, &value.repo);
    json::member(out, "label");
    json::push_text(out, &value.label);
    out.push('}');
}

/// Reads `agency.work.GitHubIssuesSource` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_git_hub_issues_source(value: &json::Value, at: &str) -> Result<crate::work::GitHubIssuesSource, json::DecodeError> {
    Ok(crate::work::GitHubIssuesSource {
        repo: {
            let at0 = json::nested(at, "repo");
            let member0 = json::member_at(value, at, "repo")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        label: {
            let at1 = json::nested(at, "label");
            let member1 = json::member_at(value, at, "label")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.work.InputId` as JSON.
pub fn encode_agency_work_input_id(value: &crate::work::InputId, out: &mut String) {
    json::push_text(out, &value.0.0);
}

/// Reads `agency.work.InputId` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_input_id(value: &json::Value, at: &str) -> Result<crate::work::InputId, json::DecodeError> {
    Ok(crate::work::InputId(crate::primitives::Uuid(json::uuid_at(value, at, "a UUID")?.to_owned())))
}

/// Writes `agency.work.WebhookSource` as JSON.
pub fn encode_agency_work_webhook_source(value: &crate::work::WebhookSource, out: &mut String) {
    out.push('{');
    json::member(out, "secret_ref");
    json::push_text(out, &value.secret_ref);
    out.push('}');
}

/// Reads `agency.work.WebhookSource` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_webhook_source(value: &json::Value, at: &str) -> Result<crate::work::WebhookSource, json::DecodeError> {
    Ok(crate::work::WebhookSource {
        secret_ref: {
            let at0 = json::nested(at, "secret_ref");
            let member0 = json::member_at(value, at, "secret_ref")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
    })
}

/// Writes `agency.work.WorkInput.State` as JSON.
pub fn encode_agency_work_work_input_state(value: &crate::work::WorkInputState, out: &mut String) {
    match value {
        crate::work::WorkInputState::Configured => json::push_text(out, "Configured"),
        crate::work::WorkInputState::Paused => json::push_text(out, "Paused"),
        crate::work::WorkInputState::Removed => json::push_text(out, "Removed"),
    }
}

/// Reads `agency.work.WorkInput.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_work_input_state(value: &json::Value, at: &str) -> Result<crate::work::WorkInputState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Configured`, `Paused`, `Removed`")? {
        "Configured" => crate::work::WorkInputState::Configured,
        "Paused" => crate::work::WorkInputState::Paused,
        "Removed" => crate::work::WorkInputState::Removed,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Configured`, `Paused`, `Removed`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.work.WorkItem.State` as JSON.
pub fn encode_agency_work_work_item_state(value: &crate::work::WorkItemState, out: &mut String) {
    match value {
        crate::work::WorkItemState::Abandoned => json::push_text(out, "Abandoned"),
        crate::work::WorkItemState::Assigned => json::push_text(out, "Assigned"),
        crate::work::WorkItemState::Done => json::push_text(out, "Done"),
        crate::work::WorkItemState::Received => json::push_text(out, "Received"),
        crate::work::WorkItemState::Returned => json::push_text(out, "Returned"),
    }
}

/// Reads `agency.work.WorkItem.State` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_work_item_state(value: &json::Value, at: &str) -> Result<crate::work::WorkItemState, json::DecodeError> {
    Ok(match json::text_at(value, at, "one of `Abandoned`, `Assigned`, `Done`, `Received`, `Returned`")? {
        "Abandoned" => crate::work::WorkItemState::Abandoned,
        "Assigned" => crate::work::WorkItemState::Assigned,
        "Done" => crate::work::WorkItemState::Done,
        "Received" => crate::work::WorkItemState::Received,
        "Returned" => crate::work::WorkItemState::Returned,
        other => return Err(json::DecodeError { at: at.to_owned(), expected: "one of `Abandoned`, `Assigned`, `Done`, `Received`, `Returned`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes `agency.work.WorkItemRef` as JSON.
pub fn encode_agency_work_work_item_ref(value: &crate::work::WorkItemRef, out: &mut String) {
    json::push_text(out, &value.0);
}

/// Reads `agency.work.WorkItemRef` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_work_item_ref(value: &json::Value, at: &str) -> Result<crate::work::WorkItemRef, json::DecodeError> {
    Ok(crate::work::WorkItemRef(json::text_at(value, at, "a string")?.to_owned()))
}

/// Writes `agency.work.WorkSource` as JSON.
pub fn encode_agency_work_work_source(value: &crate::work::WorkSource, out: &mut String) {
    match value {
        crate::work::WorkSource::A2aTask => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "A2aTask");
            out.push('}');
        }
        crate::work::WorkSource::AepStore(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "AepStore");
            json::member(out, "value");
            encode_agency_work_aep_store_source(&*held, out);
            out.push('}');
        }
        crate::work::WorkSource::GitHubIssues(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "GitHubIssues");
            json::member(out, "value");
            encode_agency_work_git_hub_issues_source(&*held, out);
            out.push('}');
        }
        crate::work::WorkSource::Manual => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Manual");
            out.push('}');
        }
        crate::work::WorkSource::Webhook(held) => {
            out.push('{');
            json::member(out, "kind");
            json::push_text(out, "Webhook");
            json::member(out, "value");
            encode_agency_work_webhook_source(&*held, out);
            out.push('}');
        }
    }
}

/// Reads `agency.work.WorkSource` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_agency_work_work_source(value: &json::Value, at: &str) -> Result<crate::work::WorkSource, json::DecodeError> {
    let tag = json::member_at(value, at, "kind")?;
    let at_tag = json::nested(at, "kind");
    Ok(match json::text_at(tag, &at_tag, "one of `A2aTask`, `AepStore`, `GitHubIssues`, `Manual`, `Webhook`")? {
        "A2aTask" => {
            if let Some(found) = value.member("value") {
                return Err(json::DecodeError::of(&json::nested(at, "value"), "no `value`: `A2aTask` carries nothing", found));
            }
            crate::work::WorkSource::A2aTask
        }
        "AepStore" => crate::work::WorkSource::AepStore({
            let at1 = json::nested(at, "value");
            let member1 = json::member_at(value, at, "value")?;
            decode_agency_work_aep_store_source(member1, &at1)?
        }),
        "GitHubIssues" => crate::work::WorkSource::GitHubIssues({
            let at2 = json::nested(at, "value");
            let member2 = json::member_at(value, at, "value")?;
            decode_agency_work_git_hub_issues_source(member2, &at2)?
        }),
        "Manual" => {
            if let Some(found) = value.member("value") {
                return Err(json::DecodeError::of(&json::nested(at, "value"), "no `value`: `Manual` carries nothing", found));
            }
            crate::work::WorkSource::Manual
        }
        "Webhook" => crate::work::WorkSource::Webhook({
            let at4 = json::nested(at, "value");
            let member4 = json::member_at(value, at, "value")?;
            decode_agency_work_webhook_source(member4, &at4)?
        }),
        other => return Err(json::DecodeError { at: at_tag.clone(), expected: "one of `A2aTask`, `AepStore`, `GitHubIssues`, `Manual`, `Webhook`".to_owned(), found: format!("`{other}`") }),
    })
}

/// Writes the event `agency.agents.AgentDefinitionCreated` as JSON.
pub fn encode_event_agency_agents_agent_definition_created(value: &crate::agents::AgentDefinitionCreated, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    out.push('}');
}

/// Writes the event `agency.agents.AgentDefinitionDeprecated` as JSON.
pub fn encode_event_agency_agents_agent_definition_deprecated(value: &crate::agents::AgentDefinitionDeprecated, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    out.push('}');
}

/// Writes the event `agency.agents.AgentDefinitionPublished` as JSON.
pub fn encode_event_agency_agents_agent_definition_published(value: &crate::agents::AgentDefinitionPublished, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    out.push('}');
}

/// Writes the event `agency.agents.AgentProvisioned` as JSON.
pub fn encode_event_agency_agents_agent_provisioned(value: &crate::agents::AgentProvisioned, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "definition_version");
    json::push_integer(out, value.definition_version);
    out.push('}');
}

/// Writes the event `agency.agents.AgentRetired` as JSON.
pub fn encode_event_agency_agents_agent_retired(value: &crate::agents::AgentRetired, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Writes the event `agency.agents.AgentSpecChecked` as JSON.
pub fn encode_event_agency_agents_agent_spec_checked(value: &crate::agents::AgentSpecChecked, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "message");
    json::push_text(out, &value.message);
    out.push('}');
}

/// Writes the event `agency.agents.AgentSpecRefused` as JSON.
pub fn encode_event_agency_agents_agent_spec_refused(value: &crate::agents::AgentSpecRefused, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "exit_code");
    json::push_integer(out, value.exit_code);
    json::member(out, "message");
    json::push_text(out, &value.message);
    out.push('}');
}

/// Writes the event `agency.auth.CredentialGranted` as JSON.
pub fn encode_event_agency_auth_credential_granted(value: &crate::auth::CredentialGranted, out: &mut String) {
    out.push('{');
    json::member(out, "grant_id");
    encode_agency_auth_grant_id(&value.grant_id, out);
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    json::member(out, "purpose");
    encode_agency_auth_grant_purpose(&value.purpose, out);
    out.push('}');
}

/// Writes the event `agency.auth.CredentialRefreshed` as JSON.
pub fn encode_event_agency_auth_credential_refreshed(value: &crate::auth::CredentialRefreshed, out: &mut String) {
    out.push('{');
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    out.push('}');
}

/// Writes the event `agency.auth.CredentialRevoked` as JSON.
pub fn encode_event_agency_auth_credential_revoked(value: &crate::auth::CredentialRevoked, out: &mut String) {
    out.push('{');
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    out.push('}');
}

/// Writes the event `agency.auth.CredentialStored` as JSON.
pub fn encode_event_agency_auth_credential_stored(value: &crate::auth::CredentialStored, out: &mut String) {
    out.push('{');
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    json::member(out, "kind");
    encode_agency_auth_credential_kind(&value.kind, out);
    json::member(out, "custody");
    encode_agency_auth_custody(&value.custody, out);
    out.push('}');
}

/// Writes the event `agency.auth.GrantRevoked` as JSON.
pub fn encode_event_agency_auth_grant_revoked(value: &crate::auth::GrantRevoked, out: &mut String) {
    out.push('{');
    json::member(out, "grant_id");
    encode_agency_auth_grant_id(&value.grant_id, out);
    out.push('}');
}

/// Writes the event `agency.auth.PolicyActivated` as JSON.
pub fn encode_event_agency_auth_policy_activated(value: &crate::auth::PolicyActivated, out: &mut String) {
    out.push('{');
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    out.push('}');
}

/// Writes the event `agency.auth.PolicyCreated` as JSON.
pub fn encode_event_agency_auth_policy_created(value: &crate::auth::PolicyCreated, out: &mut String) {
    out.push('{');
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    out.push('}');
}

/// Writes the event `agency.auth.PolicySuperseded` as JSON.
pub fn encode_event_agency_auth_policy_superseded(value: &crate::auth::PolicySuperseded, out: &mut String) {
    out.push('{');
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    out.push('}');
}

/// Writes the event `agency.auth.PrincipalDisabled` as JSON.
pub fn encode_event_agency_auth_principal_disabled(value: &crate::auth::PrincipalDisabled, out: &mut String) {
    out.push('{');
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    out.push('}');
}

/// Writes the event `agency.auth.PrincipalInvited` as JSON.
pub fn encode_event_agency_auth_principal_invited(value: &crate::auth::PrincipalInvited, out: &mut String) {
    out.push('{');
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "kind");
    encode_agency_auth_principal_kind(&value.kind, out);
    json::member(out, "subject");
    json::push_text(out, &value.subject);
    out.push('}');
}

/// Writes the event `agency.auth.RoleAssigned` as JSON.
pub fn encode_event_agency_auth_role_assigned(value: &crate::auth::RoleAssigned, out: &mut String) {
    out.push('{');
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    json::member(out, "role");
    encode_agency_auth_role(&value.role, out);
    out.push('}');
}

/// Writes the event `agency.auth.TenantCreated` as JSON.
pub fn encode_event_agency_auth_tenant_created(value: &crate::auth::TenantCreated, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    out.push('}');
}

/// Writes the event `agency.runs.RunAdmitted` as JSON.
pub fn encode_event_agency_runs_run_admitted(value: &crate::runs::RunAdmitted, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    out.push('}');
}

/// Writes the event `agency.runs.RunApprovalDenied` as JSON.
pub fn encode_event_agency_runs_run_approval_denied(value: &crate::runs::RunApprovalDenied, out: &mut String) {
    out.push('{');
    json::member(out, "approval_id");
    encode_agency_runs_approval_id(&value.approval_id, out);
    json::member(out, "answered_by");
    encode_agency_auth_principal_id(&value.answered_by, out);
    out.push('}');
}

/// Writes the event `agency.runs.RunApprovalGranted` as JSON.
pub fn encode_event_agency_runs_run_approval_granted(value: &crate::runs::RunApprovalGranted, out: &mut String) {
    out.push('{');
    json::member(out, "approval_id");
    encode_agency_runs_approval_id(&value.approval_id, out);
    json::member(out, "answered_by");
    encode_agency_auth_principal_id(&value.answered_by, out);
    out.push('}');
}

/// Writes the event `agency.runs.RunApprovalRequested` as JSON.
pub fn encode_event_agency_runs_run_approval_requested(value: &crate::runs::RunApprovalRequested, out: &mut String) {
    out.push('{');
    json::member(out, "approval_id");
    encode_agency_runs_approval_id(&value.approval_id, out);
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "subject");
    json::push_text(out, &value.subject);
    json::member(out, "capability");
    json::push_text(out, &value.capability);
    out.push('}');
}

/// Writes the event `agency.runs.RunBudgetSet` as JSON.
pub fn encode_event_agency_runs_run_budget_set(value: &crate::runs::RunBudgetSet, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "budget");
    encode_agency_shared_run_budget(&value.budget, out);
    out.push('}');
}

/// Writes the event `agency.runs.RunFailed` as JSON.
pub fn encode_event_agency_runs_run_failed(value: &crate::runs::RunFailed, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "reason");
    json::push_text(out, &value.reason);
    out.push('}');
}

/// Writes the event `agency.runs.RunProposed` as JSON.
pub fn encode_event_agency_runs_run_proposed(value: &crate::runs::RunProposed, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "usage");
    encode_agency_runs_run_usage(&value.usage, out);
    out.push('}');
}

/// Writes the event `agency.runs.RunRequested` as JSON.
pub fn encode_event_agency_runs_run_requested(value: &crate::runs::RunRequested, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    json::member(out, "work_item_ref");
    encode_agency_work_work_item_ref(&value.work_item_ref, out);
    out.push('}');
}

/// Writes the event `agency.runs.RunStarted` as JSON.
pub fn encode_event_agency_runs_run_started(value: &crate::runs::RunStarted, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "session_id");
    encode_agency_runs_loom_session_id(&value.session_id, out);
    json::member(out, "credential_source");
    encode_agency_runs_credential_source(&value.credential_source, out);
    out.push('}');
}

/// Writes the event `agency.runs.RunStopped` as JSON.
pub fn encode_event_agency_runs_run_stopped(value: &crate::runs::RunStopped, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "reason");
    json::push_text(out, &value.reason);
    out.push('}');
}

/// Writes the event `agency.service.SessionClosed` as JSON.
pub fn encode_event_agency_service_session_closed(value: &crate::service::SessionClosed, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agency_service_session_id(&value.session_id, out);
    out.push('}');
}

/// Writes the event `agency.service.SessionOpened` as JSON.
pub fn encode_event_agency_service_session_opened(value: &crate::service::SessionOpened, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agency_service_session_id(&value.session_id, out);
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    json::member(out, "kind");
    encode_agency_service_session_kind(&value.kind, out);
    out.push('}');
}

/// Writes the event `agency.teams.LeadSet` as JSON.
pub fn encode_event_agency_teams_lead_set(value: &crate::teams::LeadSet, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "lead_agent_id");
    encode_agency_agents_agent_id(&value.lead_agent_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.MemberAdded` as JSON.
pub fn encode_event_agency_teams_member_added(value: &crate::teams::MemberAdded, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.MemberRemoved` as JSON.
pub fn encode_event_agency_teams_member_removed(value: &crate::teams::MemberRemoved, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.MessageSent` as JSON.
pub fn encode_event_agency_teams_message_sent(value: &crate::teams::MessageSent, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "from_agent_id");
    encode_agency_agents_agent_id(&value.from_agent_id, out);
    json::member(out, "to");
    json::push_text(out, &value.to);
    json::member(out, "text");
    json::push_text(out, &value.text);
    out.push('}');
}

/// Writes the event `agency.teams.QueueAttached` as JSON.
pub fn encode_event_agency_teams_queue_attached(value: &crate::teams::QueueAttached, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.ReportSent` as JSON.
pub fn encode_event_agency_teams_report_sent(value: &crate::teams::ReportSent, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "from_agent_id");
    encode_agency_agents_agent_id(&value.from_agent_id, out);
    json::member(out, "status");
    json::push_text(out, &value.status);
    json::member(out, "line");
    json::push_text(out, &value.line);
    out.push('}');
}

/// Writes the event `agency.teams.SwarmDeclared` as JSON.
pub fn encode_event_agency_teams_swarm_declared(value: &crate::teams::SwarmDeclared, out: &mut String) {
    out.push('{');
    json::member(out, "swarm_id");
    encode_agency_teams_swarm_id(&value.swarm_id, out);
    json::member(out, "goal_ref");
    json::push_text(out, &value.goal_ref);
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.SwarmStopped` as JSON.
pub fn encode_event_agency_teams_swarm_stopped(value: &crate::teams::SwarmStopped, out: &mut String) {
    out.push('{');
    json::member(out, "swarm_id");
    encode_agency_teams_swarm_id(&value.swarm_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.TeamDisbanded` as JSON.
pub fn encode_event_agency_teams_team_disbanded(value: &crate::teams::TeamDisbanded, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.TeamFormed` as JSON.
pub fn encode_event_agency_teams_team_formed(value: &crate::teams::TeamFormed, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "lead_agent_id");
    encode_agency_agents_agent_id(&value.lead_agent_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.WorkforceDefined` as JSON.
pub fn encode_event_agency_teams_workforce_defined(value: &crate::teams::WorkforceDefined, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    out.push('}');
}

/// Writes the event `agency.teams.WorkforceRetired` as JSON.
pub fn encode_event_agency_teams_workforce_retired(value: &crate::teams::WorkforceRetired, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    out.push('}');
}

/// Writes the event `agency.teams.WorkforceStaffed` as JSON.
pub fn encode_event_agency_teams_workforce_staffed(value: &crate::teams::WorkforceStaffed, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    json::member(out, "teams");
    out.push('[');
    for (index0, item0) in value.teams.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_teams_team_id(&*item0, out);
    }
    out.push(']');
    json::member(out, "solo_agents");
    out.push('[');
    for (index0, item0) in value.solo_agents.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_agents_agent_id(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Writes the event `agency.tools.BindingRevoked` as JSON.
pub fn encode_event_agency_tools_binding_revoked(value: &crate::tools::BindingRevoked, out: &mut String) {
    out.push('{');
    json::member(out, "binding_id");
    encode_agency_tools_binding_id(&value.binding_id, out);
    out.push('}');
}

/// Writes the event `agency.tools.ToolBound` as JSON.
pub fn encode_event_agency_tools_tool_bound(value: &crate::tools::ToolBound, out: &mut String) {
    out.push('{');
    json::member(out, "binding_id");
    encode_agency_tools_binding_id(&value.binding_id, out);
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    json::member(out, "verbs");
    out.push('[');
    for (index0, item0) in value.verbs.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_shared_verb(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Writes the event `agency.tools.ToolRegistered` as JSON.
pub fn encode_event_agency_tools_tool_registered(value: &crate::tools::ToolRegistered, out: &mut String) {
    out.push('{');
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    json::member(out, "contract_ref");
    encode_agency_tools_contract_ref(&value.contract_ref, out);
    out.push('}');
}

/// Writes the event `agency.tools.ToolWithdrawn` as JSON.
pub fn encode_event_agency_tools_tool_withdrawn(value: &crate::tools::ToolWithdrawn, out: &mut String) {
    out.push('{');
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkInputConfigured` as JSON.
pub fn encode_event_agency_work_work_input_configured(value: &crate::work::WorkInputConfigured, out: &mut String) {
    out.push('{');
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkInputPaused` as JSON.
pub fn encode_event_agency_work_work_input_paused(value: &crate::work::WorkInputPaused, out: &mut String) {
    out.push('{');
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkInputRemoved` as JSON.
pub fn encode_event_agency_work_work_input_removed(value: &crate::work::WorkInputRemoved, out: &mut String) {
    out.push('{');
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkItemAbandoned` as JSON.
pub fn encode_event_agency_work_work_item_abandoned(value: &crate::work::WorkItemAbandoned, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkItemAssigned` as JSON.
pub fn encode_event_agency_work_work_item_assigned(value: &crate::work::WorkItemAssigned, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkItemReturned` as JSON.
pub fn encode_event_agency_work_work_item_returned(value: &crate::work::WorkItemReturned, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkItemSettled` as JSON.
pub fn encode_event_agency_work_work_item_settled(value: &crate::work::WorkItemSettled, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    out.push('}');
}

/// Writes the event `agency.work.WorkItemSubmitted` as JSON.
pub fn encode_event_agency_work_work_item_submitted(value: &crate::work::WorkItemSubmitted, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    json::member(out, "payload_digest");
    encode_agency_shared_digest(&value.payload_digest, out);
    out.push('}');
}

/// Writes the declared error `agency.agents.NotFound` as JSON.
pub fn encode_error_agency_agents_not_found(_value: &crate::agents::NotFound, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.agents.StateConflict` as JSON.
pub fn encode_error_agency_agents_state_conflict(_value: &crate::agents::StateConflict, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.auth.CredentialUnusable` as JSON.
pub fn encode_error_agency_auth_credential_unusable(_value: &crate::auth::CredentialUnusable, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.auth.GranteeInvalid` as JSON.
pub fn encode_error_agency_auth_grantee_invalid(_value: &crate::auth::GranteeInvalid, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.auth.NotFound` as JSON.
pub fn encode_error_agency_auth_not_found(_value: &crate::auth::NotFound, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.auth.StateConflict` as JSON.
pub fn encode_error_agency_auth_state_conflict(_value: &crate::auth::StateConflict, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.runs.NotFound` as JSON.
pub fn encode_error_agency_runs_not_found(_value: &crate::runs::NotFound, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.runs.RunEnded` as JSON.
pub fn encode_error_agency_runs_run_ended(_value: &crate::runs::RunEnded, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.runs.StateConflict` as JSON.
pub fn encode_error_agency_runs_state_conflict(_value: &crate::runs::StateConflict, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.service.NotFound` as JSON.
pub fn encode_error_agency_service_not_found(_value: &crate::service::NotFound, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.service.StateConflict` as JSON.
pub fn encode_error_agency_service_state_conflict(_value: &crate::service::StateConflict, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.teams.ForeignInstance` as JSON.
pub fn encode_error_agency_teams_foreign_instance(_value: &crate::teams::ForeignInstance, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.teams.NotFound` as JSON.
pub fn encode_error_agency_teams_not_found(_value: &crate::teams::NotFound, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.teams.NotOneLevelUp` as JSON.
pub fn encode_error_agency_teams_not_one_level_up(_value: &crate::teams::NotOneLevelUp, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.teams.OutsideTeam` as JSON.
pub fn encode_error_agency_teams_outside_team(_value: &crate::teams::OutsideTeam, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.teams.StateConflict` as JSON.
pub fn encode_error_agency_teams_state_conflict(_value: &crate::teams::StateConflict, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.tools.BindingTargetInvalid` as JSON.
pub fn encode_error_agency_tools_binding_target_invalid(_value: &crate::tools::BindingTargetInvalid, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.tools.NotFound` as JSON.
pub fn encode_error_agency_tools_not_found(_value: &crate::tools::NotFound, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.tools.StateConflict` as JSON.
pub fn encode_error_agency_tools_state_conflict(_value: &crate::tools::StateConflict, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.work.NotFound` as JSON.
pub fn encode_error_agency_work_not_found(_value: &crate::work::NotFound, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes the declared error `agency.work.StateConflict` as JSON.
pub fn encode_error_agency_work_state_conflict(_value: &crate::work::StateConflict, out: &mut String) {
    out.push('{');
    out.push('}');
}

/// Writes one row of the view `agency.agents.AgentDefinitionList` as JSON.
pub fn encode_view_agency_agents_agent_definition_list(value: &crate::agents::AgentDefinitionList, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "description");
    json::push_text(out, &value.description);
    json::member(out, "agent_spec");
    encode_agency_agents_agent_spec_document(&value.agent_spec, out);
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    json::member(out, "version");
    json::push_integer(out, value.version);
    json::member(out, "digest");
    encode_agency_shared_digest(&value.digest, out);
    json::member(out, "created_by");
    encode_agency_auth_principal_id(&value.created_by, out);
    if let Some(held0) = &value.check_exit_code {
        json::member(out, "check_exit_code");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.check_message {
        json::member(out, "check_message");
        json::push_text(out, &*held0);
    }
    json::member(out, "state");
    encode_agency_agents_agent_definition_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.agents.AgentList` as JSON.
pub fn encode_view_agency_agents_agent_list(value: &crate::agents::AgentList, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "definition_version");
    json::push_integer(out, value.definition_version);
    json::member(out, "display_name");
    json::push_text(out, &value.display_name);
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    if let Some(held0) = &value.current_run_id {
        json::member(out, "current_run_id");
        encode_agency_runs_run_id(&*held0, out);
    }
    json::member(out, "last_seen_at");
    json::push_text(out, &value.last_seen_at.0);
    json::member(out, "state");
    encode_agency_agents_agent_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.auth.CredentialList` as JSON.
pub fn encode_view_agency_auth_credential_list(value: &crate::auth::CredentialList, out: &mut String) {
    out.push('{');
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "kind");
    encode_agency_auth_credential_kind(&value.kind, out);
    json::member(out, "custody");
    encode_agency_auth_custody(&value.custody, out);
    json::member(out, "scope");
    encode_agency_auth_credential_scope(&value.scope, out);
    json::member(out, "owner_principal_id");
    encode_agency_auth_principal_id(&value.owner_principal_id, out);
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    json::member(out, "last_refreshed_at");
    json::push_text(out, &value.last_refreshed_at.0);
    json::member(out, "state");
    encode_agency_auth_credential_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.auth.EffectivePolicy` as JSON.
pub fn encode_view_agency_auth_effective_policy(value: &crate::auth::EffectivePolicy, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    if let Some(held0) = &value.policy_version {
        json::member(out, "policy_version");
        json::push_integer(out, *held0);
    }
    if let Some(held0) = &value.policy_digest {
        json::member(out, "policy_digest");
        encode_agency_shared_digest(&*held0, out);
    }
    if let Some(held0) = &value.rules {
        json::member(out, "rules");
        out.push('[');
        for (index1, item1) in (*held0).iter().enumerate() {
            if index1 > 0 {
                out.push(',');
            }
            encode_agency_auth_rule(&*item1, out);
        }
        out.push(']');
    }
    if let Some(held0) = &value.policy_state {
        json::member(out, "policy_state");
        encode_agency_auth_policy_state(&*held0, out);
    }
    out.push('}');
}

/// Reads the parameters of `agency.auth.EffectivePolicy` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agency_auth_effective_policy(value: &json::Value, at: &str) -> Result<(crate::agents::DefinitionId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "definition_id");
            let member0 = json::member_at(value, at, "definition_id")?;
            decode_agency_agents_definition_id(member0, &at0)?
        },
    ))
}

/// Writes one row of the view `agency.auth.GrantMatrix` as JSON.
pub fn encode_view_agency_auth_grant_matrix(value: &crate::auth::GrantMatrix, out: &mut String) {
    out.push('{');
    json::member(out, "grant_id");
    encode_agency_auth_grant_id(&value.grant_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    if let Some(held0) = &value.grantee_agent_id {
        json::member(out, "grantee_agent_id");
        encode_agency_agents_agent_id(&*held0, out);
    }
    if let Some(held0) = &value.grantee_team_id {
        json::member(out, "grantee_team_id");
        encode_agency_teams_team_id(&*held0, out);
    }
    if let Some(held0) = &value.grantee_workforce_id {
        json::member(out, "grantee_workforce_id");
        encode_agency_teams_workforce_id(&*held0, out);
    }
    json::member(out, "purpose");
    encode_agency_auth_grant_purpose(&value.purpose, out);
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    json::member(out, "state");
    encode_agency_auth_credential_grant_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.auth.PolicyList` as JSON.
pub fn encode_view_agency_auth_policy_list(value: &crate::auth::PolicyList, out: &mut String) {
    out.push('{');
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "rules");
    out.push('[');
    for (index0, item0) in value.rules.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_auth_rule(&*item0, out);
    }
    out.push(']');
    json::member(out, "version");
    json::push_integer(out, value.version);
    json::member(out, "digest");
    encode_agency_shared_digest(&value.digest, out);
    json::member(out, "state");
    encode_agency_auth_policy_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.auth.PrincipalList` as JSON.
pub fn encode_view_agency_auth_principal_list(value: &crate::auth::PrincipalList, out: &mut String) {
    out.push('{');
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "kind");
    encode_agency_auth_principal_kind(&value.kind, out);
    json::member(out, "subject");
    json::push_text(out, &value.subject);
    json::member(out, "roles");
    out.push('[');
    for (index0, item0) in value.roles.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_auth_role(&*item0, out);
    }
    out.push(']');
    if let Some(held0) = &value.mandate_ref {
        json::member(out, "mandate_ref");
        json::push_text(out, &*held0);
    }
    json::member(out, "state");
    encode_agency_auth_principal_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.runs.ApprovalInbox` as JSON.
pub fn encode_view_agency_runs_approval_inbox(value: &crate::runs::ApprovalInbox, out: &mut String) {
    out.push('{');
    json::member(out, "approval_id");
    encode_agency_runs_approval_id(&value.approval_id, out);
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "subject");
    json::push_text(out, &value.subject);
    json::member(out, "capability");
    json::push_text(out, &value.capability);
    json::member(out, "requested_at");
    json::push_text(out, &value.requested_at.0);
    if let Some(held0) = &value.answered_by {
        json::member(out, "answered_by");
        encode_agency_auth_principal_id(&*held0, out);
    }
    if let Some(held0) = &value.answered_at {
        json::member(out, "answered_at");
        json::push_text(out, &(*held0).0);
    }
    json::member(out, "state");
    encode_agency_runs_run_approval_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.runs.RunDetail` as JSON.
pub fn encode_view_agency_runs_run_detail(value: &crate::runs::RunDetail, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    json::member(out, "work_item_ref");
    encode_agency_work_work_item_ref(&value.work_item_ref, out);
    json::member(out, "budget");
    encode_agency_shared_run_budget(&value.budget, out);
    if let Some(held0) = &value.workspace {
        json::member(out, "workspace");
        encode_agency_runs_run_workspace(&*held0, out);
    }
    if let Some(held0) = &value.session_id {
        json::member(out, "session_id");
        encode_agency_runs_loom_session_id(&*held0, out);
    }
    if let Some(held0) = &value.credential_source {
        json::member(out, "credential_source");
        encode_agency_runs_credential_source(&*held0, out);
    }
    json::member(out, "started_at");
    json::push_text(out, &value.started_at.0);
    if let Some(held0) = &value.ended_at {
        json::member(out, "ended_at");
        json::push_text(out, &(*held0).0);
    }
    if let Some(held0) = &value.outcome {
        json::member(out, "outcome");
        encode_agency_runs_run_outcome(&*held0, out);
    }
    if let Some(held0) = &value.usage {
        json::member(out, "usage");
        encode_agency_runs_run_usage(&*held0, out);
    }
    json::member(out, "state");
    encode_agency_runs_run_state(&value.state, out);
    out.push('}');
}

/// Reads the parameters of `agency.runs.RunDetail` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agency_runs_run_detail(value: &json::Value, at: &str) -> Result<(crate::runs::RunId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
    ))
}

/// Writes one row of the view `agency.runs.RunList` as JSON.
pub fn encode_view_agency_runs_run_list(value: &crate::runs::RunList, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    json::member(out, "work_item_ref");
    encode_agency_work_work_item_ref(&value.work_item_ref, out);
    json::member(out, "budget");
    encode_agency_shared_run_budget(&value.budget, out);
    if let Some(held0) = &value.workspace {
        json::member(out, "workspace");
        encode_agency_runs_run_workspace(&*held0, out);
    }
    if let Some(held0) = &value.session_id {
        json::member(out, "session_id");
        encode_agency_runs_loom_session_id(&*held0, out);
    }
    if let Some(held0) = &value.credential_source {
        json::member(out, "credential_source");
        encode_agency_runs_credential_source(&*held0, out);
    }
    json::member(out, "started_at");
    json::push_text(out, &value.started_at.0);
    if let Some(held0) = &value.ended_at {
        json::member(out, "ended_at");
        json::push_text(out, &(*held0).0);
    }
    if let Some(held0) = &value.usage {
        json::member(out, "usage");
        encode_agency_runs_run_usage(&*held0, out);
    }
    json::member(out, "state");
    encode_agency_runs_run_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.runs.UsageByAgent` as JSON.
pub fn encode_view_agency_runs_usage_by_agent(value: &crate::runs::UsageByAgent, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    json::member(out, "runs");
    json::push_integer(out, value.runs);
    out.push('}');
}

/// Writes one row of the view `agency.service.SessionList` as JSON.
pub fn encode_view_agency_service_session_list(value: &crate::service::SessionList, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agency_service_session_id(&value.session_id, out);
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    json::member(out, "kind");
    encode_agency_service_session_kind(&value.kind, out);
    if let Some(held0) = &value.run_id {
        json::member(out, "run_id");
        encode_agency_runs_run_id(&*held0, out);
    }
    json::member(out, "opened_at");
    json::push_text(out, &value.opened_at.0);
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    json::member(out, "state");
    encode_agency_service_session_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.teams.SwarmList` as JSON.
pub fn encode_view_agency_teams_swarm_list(value: &crate::teams::SwarmList, out: &mut String) {
    out.push('{');
    json::member(out, "swarm_id");
    encode_agency_teams_swarm_id(&value.swarm_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "goal_ref");
    json::push_text(out, &value.goal_ref);
    json::member(out, "coordinator_agent_id");
    encode_agency_agents_agent_id(&value.coordinator_agent_id, out);
    json::member(out, "frame_digest");
    encode_agency_shared_digest(&value.frame_digest, out);
    json::member(out, "max_workers");
    json::push_integer(out, value.max_workers);
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "work_input_id");
    encode_agency_work_input_id(&value.work_input_id, out);
    json::member(out, "state");
    encode_agency_teams_swarm_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.teams.TeamDetail` as JSON.
pub fn encode_view_agency_teams_team_detail(value: &crate::teams::TeamDetail, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "lead_agent_id");
    encode_agency_agents_agent_id(&value.lead_agent_id, out);
    json::member(out, "members");
    out.push('[');
    for (index0, item0) in value.members.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_agents_agent_id(&*item0, out);
    }
    out.push(']');
    json::member(out, "broker_ref");
    json::push_text(out, &value.broker_ref);
    json::member(out, "state");
    encode_agency_teams_team_state(&value.state, out);
    out.push('}');
}

/// Reads the parameters of `agency.teams.TeamDetail` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agency_teams_team_detail(value: &json::Value, at: &str) -> Result<(crate::teams::TeamId,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
    ))
}

/// Writes one row of the view `agency.teams.TeamList` as JSON.
pub fn encode_view_agency_teams_team_list(value: &crate::teams::TeamList, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "lead_agent_id");
    encode_agency_agents_agent_id(&value.lead_agent_id, out);
    json::member(out, "members");
    out.push('[');
    for (index0, item0) in value.members.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_agents_agent_id(&*item0, out);
    }
    out.push(']');
    json::member(out, "state");
    encode_agency_teams_team_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.teams.WorkforceBoard` as JSON.
pub fn encode_view_agency_teams_workforce_board(value: &crate::teams::WorkforceBoard, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "teams");
    out.push('[');
    for (index0, item0) in value.teams.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_teams_team_id(&*item0, out);
    }
    out.push(']');
    json::member(out, "solo_agents");
    out.push('[');
    for (index0, item0) in value.solo_agents.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_agents_agent_id(&*item0, out);
    }
    out.push(']');
    json::member(out, "capacity");
    encode_agency_teams_capacity(&value.capacity, out);
    json::member(out, "state");
    encode_agency_teams_workforce_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.tools.BindingList` as JSON.
pub fn encode_view_agency_tools_binding_list(value: &crate::tools::BindingList, out: &mut String) {
    out.push('{');
    json::member(out, "binding_id");
    encode_agency_tools_binding_id(&value.binding_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    if let Some(held0) = &value.definition_id {
        json::member(out, "definition_id");
        encode_agency_agents_definition_id(&*held0, out);
    }
    if let Some(held0) = &value.team_id {
        json::member(out, "team_id");
        encode_agency_teams_team_id(&*held0, out);
    }
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    json::member(out, "verbs");
    out.push('[');
    for (index0, item0) in value.verbs.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_shared_verb(&*item0, out);
    }
    out.push(']');
    json::member(out, "scope");
    encode_agency_tools_binding_scope(&value.scope, out);
    json::member(out, "max_output_bytes");
    json::push_integer(out, value.max_output_bytes);
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    if let Some(held0) = &value.credential_id {
        json::member(out, "credential_id");
        encode_agency_auth_credential_id(&*held0, out);
    }
    json::member(out, "state");
    encode_agency_tools_tool_binding_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.tools.ToolCatalogue` as JSON.
pub fn encode_view_agency_tools_tool_catalogue(value: &crate::tools::ToolCatalogue, out: &mut String) {
    out.push('{');
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "contract_ref");
    encode_agency_tools_contract_ref(&value.contract_ref, out);
    json::member(out, "verbs");
    out.push('[');
    for (index0, item0) in value.verbs.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_shared_verb(&*item0, out);
    }
    out.push(']');
    json::member(out, "risk");
    encode_agency_tools_risk_envelope(&value.risk, out);
    if let Some(held0) = &value.implementation_ref {
        json::member(out, "implementation_ref");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.connector_instance_id {
        json::member(out, "connector_instance_id");
        json::push_text(out, &*held0);
    }
    json::member(out, "state");
    encode_agency_tools_tool_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.work.WorkInputList` as JSON.
pub fn encode_view_agency_work_work_input_list(value: &crate::work::WorkInputList, out: &mut String) {
    out.push('{');
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "source");
    encode_agency_work_work_source(&value.source, out);
    if let Some(held0) = &value.poll_interval {
        json::member(out, "poll_interval");
        json::push_text(out, &(*held0).0);
    }
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    if let Some(held0) = &value.transform_ref {
        json::member(out, "transform_ref");
        json::push_text(out, &*held0);
    }
    json::member(out, "state");
    encode_agency_work_work_input_state(&value.state, out);
    out.push('}');
}

/// Writes one row of the view `agency.work.WorkItemDetail` as JSON.
pub fn encode_view_agency_work_work_item_detail(value: &crate::work::WorkItemDetail, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    json::member(out, "title");
    json::push_text(out, &value.title);
    json::member(out, "payload_digest");
    encode_agency_shared_digest(&value.payload_digest, out);
    if let Some(held0) = &value.assigned_agent_id {
        json::member(out, "assigned_agent_id");
        encode_agency_agents_agent_id(&*held0, out);
    }
    json::member(out, "received_at");
    json::push_text(out, &value.received_at.0);
    if let Some(held0) = &value.settled_at {
        json::member(out, "settled_at");
        json::push_text(out, &(*held0).0);
    }
    json::member(out, "state");
    encode_agency_work_work_item_state(&value.state, out);
    out.push('}');
}

/// Reads the parameters of `agency.work.WorkItemDetail` from an object keyed by their wire names.
///
/// # Errors
///
/// [`json::DecodeError`] naming the parameter and what the declaration says belongs there.
pub fn decode_params_agency_work_work_item_detail(value: &json::Value, at: &str) -> Result<(crate::work::WorkItemRef,), json::DecodeError> {
    Ok((
        {
            let at0 = json::nested(at, "item_ref");
            let member0 = json::member_at(value, at, "item_ref")?;
            decode_agency_work_work_item_ref(member0, &at0)?
        },
    ))
}

/// Writes one row of the view `agency.work.WorkQueue` as JSON.
pub fn encode_view_agency_work_work_queue(value: &crate::work::WorkQueue, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    if let Some(held0) = &value.workforce_id {
        json::member(out, "workforce_id");
        encode_agency_teams_workforce_id(&*held0, out);
    }
    json::member(out, "title");
    json::push_text(out, &value.title);
    if let Some(held0) = &value.assigned_agent_id {
        json::member(out, "assigned_agent_id");
        encode_agency_agents_agent_id(&*held0, out);
    }
    json::member(out, "received_at");
    json::push_text(out, &value.received_at.0);
    json::member(out, "state");
    encode_agency_work_work_item_state(&value.state, out);
    out.push('}');
}

/// Writes the input of `agency.agents.CreateAgentDefinition` as JSON.
pub fn encode_command_agency_agents_create_agent_definition(value: &crate::agents::CreateAgentDefinition, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "description");
    json::push_text(out, &value.description);
    json::member(out, "agent_spec");
    encode_agency_agents_agent_spec_document(&value.agent_spec, out);
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    json::member(out, "created_by");
    encode_agency_auth_principal_id(&value.created_by, out);
    out.push('}');
}

/// Reads the input of `agency.agents.CreateAgentDefinition` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_agents_create_agent_definition(value: &json::Value, at: &str) -> Result<crate::agents::CreateAgentDefinition, json::DecodeError> {
    Ok(crate::agents::CreateAgentDefinition {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        name: {
            let at1 = json::nested(at, "name");
            let member1 = json::member_at(value, at, "name")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        description: {
            let at2 = json::nested(at, "description");
            let member2 = json::member_at(value, at, "description")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        agent_spec: {
            let at3 = json::nested(at, "agent_spec");
            let member3 = json::member_at(value, at, "agent_spec")?;
            decode_agency_agents_agent_spec_document(member3, &at3)?
        },
        policy_id: {
            let at4 = json::nested(at, "policy_id");
            let member4 = json::member_at(value, at, "policy_id")?;
            decode_agency_auth_policy_id(member4, &at4)?
        },
        created_by: {
            let at5 = json::nested(at, "created_by");
            let member5 = json::member_at(value, at, "created_by")?;
            decode_agency_auth_principal_id(member5, &at5)?
        },
    })
}

/// Writes the outcome of `agency.agents.CreateAgentDefinition` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_agents_create_agent_definition(value: &crate::agents::CreateAgentDefinitionOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::agents::CreateAgentDefinitionOutcome::Created { agent_definition_created } => {
            json::member(out, "outcome");
            json::push_text(out, "created");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.agents.AgentDefinitionCreated");
            json::member(out, "payload");
            encode_event_agency_agents_agent_definition_created(agent_definition_created, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.agents.DeprecateAgentDefinition` as JSON.
pub fn encode_command_agency_agents_deprecate_agent_definition(value: &crate::agents::DeprecateAgentDefinition, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    out.push('}');
}

/// Reads the input of `agency.agents.DeprecateAgentDefinition` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_agents_deprecate_agent_definition(value: &json::Value, at: &str) -> Result<crate::agents::DeprecateAgentDefinition, json::DecodeError> {
    Ok(crate::agents::DeprecateAgentDefinition {
        definition_id: {
            let at0 = json::nested(at, "definition_id");
            let member0 = json::member_at(value, at, "definition_id")?;
            decode_agency_agents_definition_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.agents.DeprecateAgentDefinition` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_agents_deprecate_agent_definition(value: &crate::agents::DeprecateAgentDefinitionOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::agents::DeprecateAgentDefinitionOutcome::NoSuchDefinition { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-definition");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.NotFound");
            json::member(out, "payload");
            encode_error_agency_agents_not_found(error, out);
            out.push('}');
        }
        crate::agents::DeprecateAgentDefinitionOutcome::Deprecated { agent_definition_deprecated } => {
            json::member(out, "outcome");
            json::push_text(out, "deprecated");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.agents.AgentDefinitionDeprecated");
            json::member(out, "payload");
            encode_event_agency_agents_agent_definition_deprecated(agent_definition_deprecated, out);
            out.push('}');
            out.push(']');
        }
        crate::agents::DeprecateAgentDefinitionOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.StateConflict");
            json::member(out, "payload");
            encode_error_agency_agents_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.agents.ProvisionAgent` as JSON.
pub fn encode_command_agency_agents_provision_agent(value: &crate::agents::ProvisionAgent, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "definition_version");
    json::push_integer(out, value.definition_version);
    json::member(out, "display_name");
    json::push_text(out, &value.display_name);
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    out.push('}');
}

/// Reads the input of `agency.agents.ProvisionAgent` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_agents_provision_agent(value: &json::Value, at: &str) -> Result<crate::agents::ProvisionAgent, json::DecodeError> {
    Ok(crate::agents::ProvisionAgent {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        definition_id: {
            let at1 = json::nested(at, "definition_id");
            let member1 = json::member_at(value, at, "definition_id")?;
            decode_agency_agents_definition_id(member1, &at1)?
        },
        definition_version: {
            let at2 = json::nested(at, "definition_version");
            let member2 = json::member_at(value, at, "definition_version")?;
            json::integer_at(member2, &at2, "an integer")?
        },
        display_name: {
            let at3 = json::nested(at, "display_name");
            let member3 = json::member_at(value, at, "display_name")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        principal_id: {
            let at4 = json::nested(at, "principal_id");
            let member4 = json::member_at(value, at, "principal_id")?;
            decode_agency_auth_principal_id(member4, &at4)?
        },
    })
}

/// Writes the outcome of `agency.agents.ProvisionAgent` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_agents_provision_agent(value: &crate::agents::ProvisionAgentOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::agents::ProvisionAgentOutcome::Provisioned { agent_provisioned } => {
            json::member(out, "outcome");
            json::push_text(out, "provisioned");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.agents.AgentProvisioned");
            json::member(out, "payload");
            encode_event_agency_agents_agent_provisioned(agent_provisioned, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.agents.PublishAgentDefinition` as JSON.
pub fn encode_command_agency_agents_publish_agent_definition(value: &crate::agents::PublishAgentDefinition, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    out.push('}');
}

/// Reads the input of `agency.agents.PublishAgentDefinition` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_agents_publish_agent_definition(value: &json::Value, at: &str) -> Result<crate::agents::PublishAgentDefinition, json::DecodeError> {
    Ok(crate::agents::PublishAgentDefinition {
        definition_id: {
            let at0 = json::nested(at, "definition_id");
            let member0 = json::member_at(value, at, "definition_id")?;
            decode_agency_agents_definition_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.agents.PublishAgentDefinition` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_agents_publish_agent_definition(value: &crate::agents::PublishAgentDefinitionOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::agents::PublishAgentDefinitionOutcome::NoSuchDefinition { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-definition");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.NotFound");
            json::member(out, "payload");
            encode_error_agency_agents_not_found(error, out);
            out.push('}');
        }
        crate::agents::PublishAgentDefinitionOutcome::Published { agent_definition_published } => {
            json::member(out, "outcome");
            json::push_text(out, "published");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.agents.AgentDefinitionPublished");
            json::member(out, "payload");
            encode_event_agency_agents_agent_definition_published(agent_definition_published, out);
            out.push('}');
            out.push(']');
        }
        crate::agents::PublishAgentDefinitionOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.StateConflict");
            json::member(out, "payload");
            encode_error_agency_agents_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.agents.RecordSpecCheck` as JSON.
pub fn encode_command_agency_agents_record_spec_check(value: &crate::agents::RecordSpecCheck, out: &mut String) {
    out.push('{');
    json::member(out, "definition_id");
    encode_agency_agents_definition_id(&value.definition_id, out);
    json::member(out, "exit_code");
    json::push_integer(out, value.exit_code);
    json::member(out, "verdict");
    encode_agency_agents_check_verdict(&value.verdict, out);
    json::member(out, "message");
    json::push_text(out, &value.message);
    out.push('}');
}

/// Reads the input of `agency.agents.RecordSpecCheck` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_agents_record_spec_check(value: &json::Value, at: &str) -> Result<crate::agents::RecordSpecCheck, json::DecodeError> {
    Ok(crate::agents::RecordSpecCheck {
        definition_id: {
            let at0 = json::nested(at, "definition_id");
            let member0 = json::member_at(value, at, "definition_id")?;
            decode_agency_agents_definition_id(member0, &at0)?
        },
        exit_code: {
            let at1 = json::nested(at, "exit_code");
            let member1 = json::member_at(value, at, "exit_code")?;
            json::integer_at(member1, &at1, "an integer")?
        },
        verdict: {
            let at2 = json::nested(at, "verdict");
            let member2 = json::member_at(value, at, "verdict")?;
            decode_agency_agents_check_verdict(member2, &at2)?
        },
        message: {
            let at3 = json::nested(at, "message");
            let member3 = json::member_at(value, at, "message")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
    })
}

/// Writes the outcome of `agency.agents.RecordSpecCheck` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_agents_record_spec_check(value: &crate::agents::RecordSpecCheckOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::agents::RecordSpecCheckOutcome::NoSuchDefinition { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-definition");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.NotFound");
            json::member(out, "payload");
            encode_error_agency_agents_not_found(error, out);
            out.push('}');
        }
        crate::agents::RecordSpecCheckOutcome::Refused { agent_spec_refused } => {
            json::member(out, "outcome");
            json::push_text(out, "refused");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.agents.AgentSpecRefused");
            json::member(out, "payload");
            encode_event_agency_agents_agent_spec_refused(agent_spec_refused, out);
            out.push('}');
            out.push(']');
        }
        crate::agents::RecordSpecCheckOutcome::Checked { agent_spec_checked } => {
            json::member(out, "outcome");
            json::push_text(out, "checked");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.agents.AgentSpecChecked");
            json::member(out, "payload");
            encode_event_agency_agents_agent_spec_checked(agent_spec_checked, out);
            out.push('}');
            out.push(']');
        }
        crate::agents::RecordSpecCheckOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.StateConflict");
            json::member(out, "payload");
            encode_error_agency_agents_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.agents.RetireAgent` as JSON.
pub fn encode_command_agency_agents_retire_agent(value: &crate::agents::RetireAgent, out: &mut String) {
    out.push('{');
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Reads the input of `agency.agents.RetireAgent` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_agents_retire_agent(value: &json::Value, at: &str) -> Result<crate::agents::RetireAgent, json::DecodeError> {
    Ok(crate::agents::RetireAgent {
        agent_id: {
            let at0 = json::nested(at, "agent_id");
            let member0 = json::member_at(value, at, "agent_id")?;
            decode_agency_agents_agent_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.agents.RetireAgent` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_agents_retire_agent(value: &crate::agents::RetireAgentOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::agents::RetireAgentOutcome::NoSuchAgent { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-agent");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.NotFound");
            json::member(out, "payload");
            encode_error_agency_agents_not_found(error, out);
            out.push('}');
        }
        crate::agents::RetireAgentOutcome::Retired { agent_retired } => {
            json::member(out, "outcome");
            json::push_text(out, "retired");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.agents.AgentRetired");
            json::member(out, "payload");
            encode_event_agency_agents_agent_retired(agent_retired, out);
            out.push('}');
            out.push(']');
        }
        crate::agents::RetireAgentOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.agents.StateConflict");
            json::member(out, "payload");
            encode_error_agency_agents_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.ActivatePolicy` as JSON.
pub fn encode_command_agency_auth_activate_policy(value: &crate::auth::ActivatePolicy, out: &mut String) {
    out.push('{');
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    out.push('}');
}

/// Reads the input of `agency.auth.ActivatePolicy` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_activate_policy(value: &json::Value, at: &str) -> Result<crate::auth::ActivatePolicy, json::DecodeError> {
    Ok(crate::auth::ActivatePolicy {
        policy_id: {
            let at0 = json::nested(at, "policy_id");
            let member0 = json::member_at(value, at, "policy_id")?;
            decode_agency_auth_policy_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.auth.ActivatePolicy` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_activate_policy(value: &crate::auth::ActivatePolicyOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::ActivatePolicyOutcome::NoSuchPolicy { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-policy");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.NotFound");
            json::member(out, "payload");
            encode_error_agency_auth_not_found(error, out);
            out.push('}');
        }
        crate::auth::ActivatePolicyOutcome::Activated { policy_activated } => {
            json::member(out, "outcome");
            json::push_text(out, "activated");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.PolicyActivated");
            json::member(out, "payload");
            encode_event_agency_auth_policy_activated(policy_activated, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::ActivatePolicyOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.StateConflict");
            json::member(out, "payload");
            encode_error_agency_auth_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.AssignRole` as JSON.
pub fn encode_command_agency_auth_assign_role(value: &crate::auth::AssignRole, out: &mut String) {
    out.push('{');
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    json::member(out, "role");
    encode_agency_auth_role(&value.role, out);
    out.push('}');
}

/// Reads the input of `agency.auth.AssignRole` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_assign_role(value: &json::Value, at: &str) -> Result<crate::auth::AssignRole, json::DecodeError> {
    Ok(crate::auth::AssignRole {
        principal_id: {
            let at0 = json::nested(at, "principal_id");
            let member0 = json::member_at(value, at, "principal_id")?;
            decode_agency_auth_principal_id(member0, &at0)?
        },
        role: {
            let at1 = json::nested(at, "role");
            let member1 = json::member_at(value, at, "role")?;
            decode_agency_auth_role(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.auth.AssignRole` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_assign_role(value: &crate::auth::AssignRoleOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::AssignRoleOutcome::NoSuchPrincipal { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-principal");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.NotFound");
            json::member(out, "payload");
            encode_error_agency_auth_not_found(error, out);
            out.push('}');
        }
        crate::auth::AssignRoleOutcome::Assigned { role_assigned } => {
            json::member(out, "outcome");
            json::push_text(out, "assigned");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.RoleAssigned");
            json::member(out, "payload");
            encode_event_agency_auth_role_assigned(role_assigned, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.CreatePolicy` as JSON.
pub fn encode_command_agency_auth_create_policy(value: &crate::auth::CreatePolicy, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "rules");
    out.push('[');
    for (index0, item0) in value.rules.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_auth_rule(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads the input of `agency.auth.CreatePolicy` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_create_policy(value: &json::Value, at: &str) -> Result<crate::auth::CreatePolicy, json::DecodeError> {
    Ok(crate::auth::CreatePolicy {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        name: {
            let at1 = json::nested(at, "name");
            let member1 = json::member_at(value, at, "name")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        rules: {
            let at2 = json::nested(at, "rules");
            let member2 = json::member_at(value, at, "rules")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agency_auth_rule(element2, &nested2)?);
                }
                items2
            }
        },
    })
}

/// Writes the outcome of `agency.auth.CreatePolicy` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_create_policy(value: &crate::auth::CreatePolicyOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::CreatePolicyOutcome::Created { policy_created } => {
            json::member(out, "outcome");
            json::push_text(out, "created");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.PolicyCreated");
            json::member(out, "payload");
            encode_event_agency_auth_policy_created(policy_created, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.CreateTenant` as JSON.
pub fn encode_command_agency_auth_create_tenant(value: &crate::auth::CreateTenant, out: &mut String) {
    out.push('{');
    json::member(out, "name");
    json::push_text(out, &value.name);
    if let Some(held0) = &value.mandate_tenant_ref {
        json::member(out, "mandate_tenant_ref");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads the input of `agency.auth.CreateTenant` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_create_tenant(value: &json::Value, at: &str) -> Result<crate::auth::CreateTenant, json::DecodeError> {
    Ok(crate::auth::CreateTenant {
        name: {
            let at0 = json::nested(at, "name");
            let member0 = json::member_at(value, at, "name")?;
            json::text_at(member0, &at0, "a string")?.to_owned()
        },
        mandate_tenant_ref: match value.member("mandate_tenant_ref") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "mandate_tenant_ref");
                Some(json::text_at(member1, &at1, "a string")?.to_owned())
            }
        },
    })
}

/// Writes the outcome of `agency.auth.CreateTenant` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_create_tenant(value: &crate::auth::CreateTenantOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::CreateTenantOutcome::Created { tenant_created } => {
            json::member(out, "outcome");
            json::push_text(out, "created");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.TenantCreated");
            json::member(out, "payload");
            encode_event_agency_auth_tenant_created(tenant_created, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.DisablePrincipal` as JSON.
pub fn encode_command_agency_auth_disable_principal(value: &crate::auth::DisablePrincipal, out: &mut String) {
    out.push('{');
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    out.push('}');
}

/// Reads the input of `agency.auth.DisablePrincipal` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_disable_principal(value: &json::Value, at: &str) -> Result<crate::auth::DisablePrincipal, json::DecodeError> {
    Ok(crate::auth::DisablePrincipal {
        principal_id: {
            let at0 = json::nested(at, "principal_id");
            let member0 = json::member_at(value, at, "principal_id")?;
            decode_agency_auth_principal_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.auth.DisablePrincipal` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_disable_principal(value: &crate::auth::DisablePrincipalOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::DisablePrincipalOutcome::NoSuchPrincipal { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-principal");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.NotFound");
            json::member(out, "payload");
            encode_error_agency_auth_not_found(error, out);
            out.push('}');
        }
        crate::auth::DisablePrincipalOutcome::Disabled { principal_disabled } => {
            json::member(out, "outcome");
            json::push_text(out, "disabled");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.PrincipalDisabled");
            json::member(out, "payload");
            encode_event_agency_auth_principal_disabled(principal_disabled, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::DisablePrincipalOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.StateConflict");
            json::member(out, "payload");
            encode_error_agency_auth_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.GrantCredential` as JSON.
pub fn encode_command_agency_auth_grant_credential(value: &crate::auth::GrantCredential, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    if let Some(held0) = &value.grantee_agent_id {
        json::member(out, "grantee_agent_id");
        encode_agency_agents_agent_id(&*held0, out);
    }
    if let Some(held0) = &value.grantee_team_id {
        json::member(out, "grantee_team_id");
        encode_agency_teams_team_id(&*held0, out);
    }
    if let Some(held0) = &value.grantee_workforce_id {
        json::member(out, "grantee_workforce_id");
        encode_agency_teams_workforce_id(&*held0, out);
    }
    json::member(out, "purpose");
    encode_agency_auth_grant_purpose(&value.purpose, out);
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    out.push('}');
}

/// Reads the input of `agency.auth.GrantCredential` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_grant_credential(value: &json::Value, at: &str) -> Result<crate::auth::GrantCredential, json::DecodeError> {
    Ok(crate::auth::GrantCredential {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        credential_id: {
            let at1 = json::nested(at, "credential_id");
            let member1 = json::member_at(value, at, "credential_id")?;
            decode_agency_auth_credential_id(member1, &at1)?
        },
        grantee_agent_id: match value.member("grantee_agent_id") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "grantee_agent_id");
                Some(decode_agency_agents_agent_id(member2, &at2)?)
            }
        },
        grantee_team_id: match value.member("grantee_team_id") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "grantee_team_id");
                Some(decode_agency_teams_team_id(member3, &at3)?)
            }
        },
        grantee_workforce_id: match value.member("grantee_workforce_id") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "grantee_workforce_id");
                Some(decode_agency_teams_workforce_id(member4, &at4)?)
            }
        },
        purpose: {
            let at5 = json::nested(at, "purpose");
            let member5 = json::member_at(value, at, "purpose")?;
            decode_agency_auth_grant_purpose(member5, &at5)?
        },
        expires_at: match value.member("expires_at") {
            None | Some(json::Value::Null) => None,
            Some(member6) => {
                let at6 = json::nested(at, "expires_at");
                Some(crate::primitives::Timestamp(json::text_at(member6, &at6, "an RFC 3339 instant")?.to_owned()))
            }
        },
    })
}

/// Writes the outcome of `agency.auth.GrantCredential` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_grant_credential(value: &crate::auth::GrantCredentialOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::GrantCredentialOutcome::GrantedToAgent { credential_granted } => {
            json::member(out, "outcome");
            json::push_text(out, "granted-to-agent");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.CredentialGranted");
            json::member(out, "payload");
            encode_event_agency_auth_credential_granted(credential_granted, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::GrantCredentialOutcome::GrantedToTeam { credential_granted } => {
            json::member(out, "outcome");
            json::push_text(out, "granted-to-team");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.CredentialGranted");
            json::member(out, "payload");
            encode_event_agency_auth_credential_granted(credential_granted, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::GrantCredentialOutcome::GrantedToWorkforce { credential_granted } => {
            json::member(out, "outcome");
            json::push_text(out, "granted-to-workforce");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.CredentialGranted");
            json::member(out, "payload");
            encode_event_agency_auth_credential_granted(credential_granted, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::GrantCredentialOutcome::GranteeInvalid { error } => {
            json::member(out, "outcome");
            json::push_text(out, "grantee-invalid");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.GranteeInvalid");
            json::member(out, "payload");
            encode_error_agency_auth_grantee_invalid(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.InvitePrincipal` as JSON.
pub fn encode_command_agency_auth_invite_principal(value: &crate::auth::InvitePrincipal, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "kind");
    encode_agency_auth_principal_kind(&value.kind, out);
    json::member(out, "subject");
    json::push_text(out, &value.subject);
    json::member(out, "roles");
    out.push('[');
    for (index0, item0) in value.roles.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_auth_role(&*item0, out);
    }
    out.push(']');
    if let Some(held0) = &value.mandate_ref {
        json::member(out, "mandate_ref");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads the input of `agency.auth.InvitePrincipal` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_invite_principal(value: &json::Value, at: &str) -> Result<crate::auth::InvitePrincipal, json::DecodeError> {
    Ok(crate::auth::InvitePrincipal {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        kind: {
            let at1 = json::nested(at, "kind");
            let member1 = json::member_at(value, at, "kind")?;
            decode_agency_auth_principal_kind(member1, &at1)?
        },
        subject: {
            let at2 = json::nested(at, "subject");
            let member2 = json::member_at(value, at, "subject")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        roles: {
            let at3 = json::nested(at, "roles");
            let member3 = json::member_at(value, at, "roles")?;
            {
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(decode_agency_auth_role(element3, &nested3)?);
                }
                items3
            }
        },
        mandate_ref: match value.member("mandate_ref") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "mandate_ref");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
    })
}

/// Writes the outcome of `agency.auth.InvitePrincipal` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_invite_principal(value: &crate::auth::InvitePrincipalOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::InvitePrincipalOutcome::Invited { principal_invited } => {
            json::member(out, "outcome");
            json::push_text(out, "invited");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.PrincipalInvited");
            json::member(out, "payload");
            encode_event_agency_auth_principal_invited(principal_invited, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.RefreshCredential` as JSON.
pub fn encode_command_agency_auth_refresh_credential(value: &crate::auth::RefreshCredential, out: &mut String) {
    out.push('{');
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    out.push('}');
}

/// Reads the input of `agency.auth.RefreshCredential` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_refresh_credential(value: &json::Value, at: &str) -> Result<crate::auth::RefreshCredential, json::DecodeError> {
    Ok(crate::auth::RefreshCredential {
        credential_id: {
            let at0 = json::nested(at, "credential_id");
            let member0 = json::member_at(value, at, "credential_id")?;
            decode_agency_auth_credential_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.auth.RefreshCredential` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_refresh_credential(value: &crate::auth::RefreshCredentialOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::RefreshCredentialOutcome::NoSuchCredential { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-credential");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.NotFound");
            json::member(out, "payload");
            encode_error_agency_auth_not_found(error, out);
            out.push('}');
        }
        crate::auth::RefreshCredentialOutcome::Revoked { error } => {
            json::member(out, "outcome");
            json::push_text(out, "revoked");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.CredentialUnusable");
            json::member(out, "payload");
            encode_error_agency_auth_credential_unusable(error, out);
            out.push('}');
        }
        crate::auth::RefreshCredentialOutcome::Refreshed { credential_refreshed } => {
            json::member(out, "outcome");
            json::push_text(out, "refreshed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.CredentialRefreshed");
            json::member(out, "payload");
            encode_event_agency_auth_credential_refreshed(credential_refreshed, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.RevokeCredential` as JSON.
pub fn encode_command_agency_auth_revoke_credential(value: &crate::auth::RevokeCredential, out: &mut String) {
    out.push('{');
    json::member(out, "credential_id");
    encode_agency_auth_credential_id(&value.credential_id, out);
    out.push('}');
}

/// Reads the input of `agency.auth.RevokeCredential` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_revoke_credential(value: &json::Value, at: &str) -> Result<crate::auth::RevokeCredential, json::DecodeError> {
    Ok(crate::auth::RevokeCredential {
        credential_id: {
            let at0 = json::nested(at, "credential_id");
            let member0 = json::member_at(value, at, "credential_id")?;
            decode_agency_auth_credential_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.auth.RevokeCredential` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_revoke_credential(value: &crate::auth::RevokeCredentialOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::RevokeCredentialOutcome::NoSuchCredential { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-credential");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.NotFound");
            json::member(out, "payload");
            encode_error_agency_auth_not_found(error, out);
            out.push('}');
        }
        crate::auth::RevokeCredentialOutcome::Revoked { credential_revoked } => {
            json::member(out, "outcome");
            json::push_text(out, "revoked");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.CredentialRevoked");
            json::member(out, "payload");
            encode_event_agency_auth_credential_revoked(credential_revoked, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::RevokeCredentialOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.StateConflict");
            json::member(out, "payload");
            encode_error_agency_auth_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.RevokeGrant` as JSON.
pub fn encode_command_agency_auth_revoke_grant(value: &crate::auth::RevokeGrant, out: &mut String) {
    out.push('{');
    json::member(out, "grant_id");
    encode_agency_auth_grant_id(&value.grant_id, out);
    out.push('}');
}

/// Reads the input of `agency.auth.RevokeGrant` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_revoke_grant(value: &json::Value, at: &str) -> Result<crate::auth::RevokeGrant, json::DecodeError> {
    Ok(crate::auth::RevokeGrant {
        grant_id: {
            let at0 = json::nested(at, "grant_id");
            let member0 = json::member_at(value, at, "grant_id")?;
            decode_agency_auth_grant_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.auth.RevokeGrant` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_revoke_grant(value: &crate::auth::RevokeGrantOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::RevokeGrantOutcome::NoSuchGrant { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-grant");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.NotFound");
            json::member(out, "payload");
            encode_error_agency_auth_not_found(error, out);
            out.push('}');
        }
        crate::auth::RevokeGrantOutcome::Revoked { grant_revoked } => {
            json::member(out, "outcome");
            json::push_text(out, "revoked");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.GrantRevoked");
            json::member(out, "payload");
            encode_event_agency_auth_grant_revoked(grant_revoked, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::RevokeGrantOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.StateConflict");
            json::member(out, "payload");
            encode_error_agency_auth_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.StoreCredential` as JSON.
pub fn encode_command_agency_auth_store_credential(value: &crate::auth::StoreCredential, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "kind");
    encode_agency_auth_credential_kind(&value.kind, out);
    json::member(out, "custody");
    encode_agency_auth_custody(&value.custody, out);
    json::member(out, "scope");
    encode_agency_auth_credential_scope(&value.scope, out);
    json::member(out, "owner_principal_id");
    encode_agency_auth_principal_id(&value.owner_principal_id, out);
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    out.push('}');
}

/// Reads the input of `agency.auth.StoreCredential` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_store_credential(value: &json::Value, at: &str) -> Result<crate::auth::StoreCredential, json::DecodeError> {
    Ok(crate::auth::StoreCredential {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        kind: {
            let at1 = json::nested(at, "kind");
            let member1 = json::member_at(value, at, "kind")?;
            decode_agency_auth_credential_kind(member1, &at1)?
        },
        custody: {
            let at2 = json::nested(at, "custody");
            let member2 = json::member_at(value, at, "custody")?;
            decode_agency_auth_custody(member2, &at2)?
        },
        scope: {
            let at3 = json::nested(at, "scope");
            let member3 = json::member_at(value, at, "scope")?;
            decode_agency_auth_credential_scope(member3, &at3)?
        },
        owner_principal_id: {
            let at4 = json::nested(at, "owner_principal_id");
            let member4 = json::member_at(value, at, "owner_principal_id")?;
            decode_agency_auth_principal_id(member4, &at4)?
        },
        expires_at: match value.member("expires_at") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "expires_at");
                Some(crate::primitives::Timestamp(json::text_at(member5, &at5, "an RFC 3339 instant")?.to_owned()))
            }
        },
    })
}

/// Writes the outcome of `agency.auth.StoreCredential` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_store_credential(value: &crate::auth::StoreCredentialOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::StoreCredentialOutcome::Stored { credential_stored } => {
            json::member(out, "outcome");
            json::push_text(out, "stored");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.CredentialStored");
            json::member(out, "payload");
            encode_event_agency_auth_credential_stored(credential_stored, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.auth.SupersedePolicy` as JSON.
pub fn encode_command_agency_auth_supersede_policy(value: &crate::auth::SupersedePolicy, out: &mut String) {
    out.push('{');
    json::member(out, "policy_id");
    encode_agency_auth_policy_id(&value.policy_id, out);
    out.push('}');
}

/// Reads the input of `agency.auth.SupersedePolicy` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_auth_supersede_policy(value: &json::Value, at: &str) -> Result<crate::auth::SupersedePolicy, json::DecodeError> {
    Ok(crate::auth::SupersedePolicy {
        policy_id: {
            let at0 = json::nested(at, "policy_id");
            let member0 = json::member_at(value, at, "policy_id")?;
            decode_agency_auth_policy_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.auth.SupersedePolicy` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_auth_supersede_policy(value: &crate::auth::SupersedePolicyOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::auth::SupersedePolicyOutcome::NoSuchPolicy { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-policy");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.NotFound");
            json::member(out, "payload");
            encode_error_agency_auth_not_found(error, out);
            out.push('}');
        }
        crate::auth::SupersedePolicyOutcome::Superseded { policy_superseded } => {
            json::member(out, "outcome");
            json::push_text(out, "superseded");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.auth.PolicySuperseded");
            json::member(out, "payload");
            encode_event_agency_auth_policy_superseded(policy_superseded, out);
            out.push('}');
            out.push(']');
        }
        crate::auth::SupersedePolicyOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.auth.StateConflict");
            json::member(out, "payload");
            encode_error_agency_auth_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.AdmitRun` as JSON.
pub fn encode_command_agency_runs_admit_run(value: &crate::runs::AdmitRun, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    out.push('}');
}

/// Reads the input of `agency.runs.AdmitRun` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_admit_run(value: &json::Value, at: &str) -> Result<crate::runs::AdmitRun, json::DecodeError> {
    Ok(crate::runs::AdmitRun {
        run_id: {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.runs.AdmitRun` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_admit_run(value: &crate::runs::AdmitRunOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::AdmitRunOutcome::NoSuchRun { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-run");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::AdmitRunOutcome::Admitted { run_admitted } => {
            json::member(out, "outcome");
            json::push_text(out, "admitted");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunAdmitted");
            json::member(out, "payload");
            encode_event_agency_runs_run_admitted(run_admitted, out);
            out.push('}');
            out.push(']');
        }
        crate::runs::AdmitRunOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.StateConflict");
            json::member(out, "payload");
            encode_error_agency_runs_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.DenyRunApproval` as JSON.
pub fn encode_command_agency_runs_deny_run_approval(value: &crate::runs::DenyRunApproval, out: &mut String) {
    out.push('{');
    json::member(out, "approval_id");
    encode_agency_runs_approval_id(&value.approval_id, out);
    json::member(out, "answered_by");
    encode_agency_auth_principal_id(&value.answered_by, out);
    out.push('}');
}

/// Reads the input of `agency.runs.DenyRunApproval` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_deny_run_approval(value: &json::Value, at: &str) -> Result<crate::runs::DenyRunApproval, json::DecodeError> {
    Ok(crate::runs::DenyRunApproval {
        approval_id: {
            let at0 = json::nested(at, "approval_id");
            let member0 = json::member_at(value, at, "approval_id")?;
            decode_agency_runs_approval_id(member0, &at0)?
        },
        answered_by: {
            let at1 = json::nested(at, "answered_by");
            let member1 = json::member_at(value, at, "answered_by")?;
            decode_agency_auth_principal_id(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.runs.DenyRunApproval` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_deny_run_approval(value: &crate::runs::DenyRunApprovalOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::DenyRunApprovalOutcome::NoSuchApproval { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-approval");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::DenyRunApprovalOutcome::Denied { run_approval_denied } => {
            json::member(out, "outcome");
            json::push_text(out, "denied");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunApprovalDenied");
            json::member(out, "payload");
            encode_event_agency_runs_run_approval_denied(run_approval_denied, out);
            out.push('}');
            out.push(']');
        }
        crate::runs::DenyRunApprovalOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.StateConflict");
            json::member(out, "payload");
            encode_error_agency_runs_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.GrantRunApproval` as JSON.
pub fn encode_command_agency_runs_grant_run_approval(value: &crate::runs::GrantRunApproval, out: &mut String) {
    out.push('{');
    json::member(out, "approval_id");
    encode_agency_runs_approval_id(&value.approval_id, out);
    json::member(out, "answered_by");
    encode_agency_auth_principal_id(&value.answered_by, out);
    out.push('}');
}

/// Reads the input of `agency.runs.GrantRunApproval` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_grant_run_approval(value: &json::Value, at: &str) -> Result<crate::runs::GrantRunApproval, json::DecodeError> {
    Ok(crate::runs::GrantRunApproval {
        approval_id: {
            let at0 = json::nested(at, "approval_id");
            let member0 = json::member_at(value, at, "approval_id")?;
            decode_agency_runs_approval_id(member0, &at0)?
        },
        answered_by: {
            let at1 = json::nested(at, "answered_by");
            let member1 = json::member_at(value, at, "answered_by")?;
            decode_agency_auth_principal_id(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.runs.GrantRunApproval` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_grant_run_approval(value: &crate::runs::GrantRunApprovalOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::GrantRunApprovalOutcome::NoSuchApproval { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-approval");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::GrantRunApprovalOutcome::Granted { run_approval_granted } => {
            json::member(out, "outcome");
            json::push_text(out, "granted");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunApprovalGranted");
            json::member(out, "payload");
            encode_event_agency_runs_run_approval_granted(run_approval_granted, out);
            out.push('}');
            out.push(']');
        }
        crate::runs::GrantRunApprovalOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.StateConflict");
            json::member(out, "payload");
            encode_error_agency_runs_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.RecordRunFailure` as JSON.
pub fn encode_command_agency_runs_record_run_failure(value: &crate::runs::RecordRunFailure, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "reason");
    json::push_text(out, &value.reason);
    out.push('}');
}

/// Reads the input of `agency.runs.RecordRunFailure` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_record_run_failure(value: &json::Value, at: &str) -> Result<crate::runs::RecordRunFailure, json::DecodeError> {
    Ok(crate::runs::RecordRunFailure {
        run_id: {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
        reason: {
            let at1 = json::nested(at, "reason");
            let member1 = json::member_at(value, at, "reason")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes the outcome of `agency.runs.RecordRunFailure` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_record_run_failure(value: &crate::runs::RecordRunFailureOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::RecordRunFailureOutcome::NoSuchRun { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-run");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::RecordRunFailureOutcome::Failed { run_failed } => {
            json::member(out, "outcome");
            json::push_text(out, "failed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunFailed");
            json::member(out, "payload");
            encode_event_agency_runs_run_failed(run_failed, out);
            out.push('}');
            out.push(']');
        }
        crate::runs::RecordRunFailureOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.StateConflict");
            json::member(out, "payload");
            encode_error_agency_runs_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.RecordRunProposal` as JSON.
pub fn encode_command_agency_runs_record_run_proposal(value: &crate::runs::RecordRunProposal, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "usage");
    encode_agency_runs_run_usage(&value.usage, out);
    out.push('}');
}

/// Reads the input of `agency.runs.RecordRunProposal` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_record_run_proposal(value: &json::Value, at: &str) -> Result<crate::runs::RecordRunProposal, json::DecodeError> {
    Ok(crate::runs::RecordRunProposal {
        run_id: {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
        usage: {
            let at1 = json::nested(at, "usage");
            let member1 = json::member_at(value, at, "usage")?;
            decode_agency_runs_run_usage(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.runs.RecordRunProposal` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_record_run_proposal(value: &crate::runs::RecordRunProposalOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::RecordRunProposalOutcome::NoSuchRun { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-run");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::RecordRunProposalOutcome::Proposed { run_proposed } => {
            json::member(out, "outcome");
            json::push_text(out, "proposed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunProposed");
            json::member(out, "payload");
            encode_event_agency_runs_run_proposed(run_proposed, out);
            out.push('}');
            out.push(']');
        }
        crate::runs::RecordRunProposalOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.StateConflict");
            json::member(out, "payload");
            encode_error_agency_runs_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.RecordRunStarted` as JSON.
pub fn encode_command_agency_runs_record_run_started(value: &crate::runs::RecordRunStarted, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "session_id");
    encode_agency_runs_loom_session_id(&value.session_id, out);
    json::member(out, "credential_source");
    encode_agency_runs_credential_source(&value.credential_source, out);
    out.push('}');
}

/// Reads the input of `agency.runs.RecordRunStarted` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_record_run_started(value: &json::Value, at: &str) -> Result<crate::runs::RecordRunStarted, json::DecodeError> {
    Ok(crate::runs::RecordRunStarted {
        run_id: {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
        session_id: {
            let at1 = json::nested(at, "session_id");
            let member1 = json::member_at(value, at, "session_id")?;
            decode_agency_runs_loom_session_id(member1, &at1)?
        },
        credential_source: {
            let at2 = json::nested(at, "credential_source");
            let member2 = json::member_at(value, at, "credential_source")?;
            decode_agency_runs_credential_source(member2, &at2)?
        },
    })
}

/// Writes the outcome of `agency.runs.RecordRunStarted` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_record_run_started(value: &crate::runs::RecordRunStartedOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::RecordRunStartedOutcome::NoSuchRun { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-run");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::RecordRunStartedOutcome::Started { run_started } => {
            json::member(out, "outcome");
            json::push_text(out, "started");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunStarted");
            json::member(out, "payload");
            encode_event_agency_runs_run_started(run_started, out);
            out.push('}');
            out.push(']');
        }
        crate::runs::RecordRunStartedOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.StateConflict");
            json::member(out, "payload");
            encode_error_agency_runs_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.RequestRunApproval` as JSON.
pub fn encode_command_agency_runs_request_run_approval(value: &crate::runs::RequestRunApproval, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "subject");
    json::push_text(out, &value.subject);
    json::member(out, "capability");
    json::push_text(out, &value.capability);
    out.push('}');
}

/// Reads the input of `agency.runs.RequestRunApproval` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_request_run_approval(value: &json::Value, at: &str) -> Result<crate::runs::RequestRunApproval, json::DecodeError> {
    Ok(crate::runs::RequestRunApproval {
        run_id: {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
        subject: {
            let at1 = json::nested(at, "subject");
            let member1 = json::member_at(value, at, "subject")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        capability: {
            let at2 = json::nested(at, "capability");
            let member2 = json::member_at(value, at, "capability")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
    })
}

/// Writes the outcome of `agency.runs.RequestRunApproval` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_request_run_approval(value: &crate::runs::RequestRunApprovalOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::RequestRunApprovalOutcome::Requested { run_approval_requested } => {
            json::member(out, "outcome");
            json::push_text(out, "requested");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunApprovalRequested");
            json::member(out, "payload");
            encode_event_agency_runs_run_approval_requested(run_approval_requested, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.SetRunBudget` as JSON.
pub fn encode_command_agency_runs_set_run_budget(value: &crate::runs::SetRunBudget, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "budget");
    encode_agency_shared_run_budget(&value.budget, out);
    out.push('}');
}

/// Reads the input of `agency.runs.SetRunBudget` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_set_run_budget(value: &json::Value, at: &str) -> Result<crate::runs::SetRunBudget, json::DecodeError> {
    Ok(crate::runs::SetRunBudget {
        run_id: {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
        budget: {
            let at1 = json::nested(at, "budget");
            let member1 = json::member_at(value, at, "budget")?;
            decode_agency_shared_run_budget(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.runs.SetRunBudget` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_set_run_budget(value: &crate::runs::SetRunBudgetOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::SetRunBudgetOutcome::NoSuchRun { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-run");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::SetRunBudgetOutcome::Ended { error } => {
            json::member(out, "outcome");
            json::push_text(out, "ended");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.RunEnded");
            json::member(out, "payload");
            encode_error_agency_runs_run_ended(error, out);
            out.push('}');
        }
        crate::runs::SetRunBudgetOutcome::Set { run_budget_set } => {
            json::member(out, "outcome");
            json::push_text(out, "set");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunBudgetSet");
            json::member(out, "payload");
            encode_event_agency_runs_run_budget_set(run_budget_set, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.StartRun` as JSON.
pub fn encode_command_agency_runs_start_run(value: &crate::runs::StartRun, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    json::member(out, "work_item_ref");
    encode_agency_work_work_item_ref(&value.work_item_ref, out);
    json::member(out, "budget");
    encode_agency_shared_run_budget(&value.budget, out);
    if let Some(held0) = &value.workspace {
        json::member(out, "workspace");
        encode_agency_runs_run_workspace(&*held0, out);
    }
    out.push('}');
}

/// Reads the input of `agency.runs.StartRun` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_start_run(value: &json::Value, at: &str) -> Result<crate::runs::StartRun, json::DecodeError> {
    Ok(crate::runs::StartRun {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agency_agents_agent_id(member1, &at1)?
        },
        work_item_ref: {
            let at2 = json::nested(at, "work_item_ref");
            let member2 = json::member_at(value, at, "work_item_ref")?;
            decode_agency_work_work_item_ref(member2, &at2)?
        },
        budget: {
            let at3 = json::nested(at, "budget");
            let member3 = json::member_at(value, at, "budget")?;
            decode_agency_shared_run_budget(member3, &at3)?
        },
        workspace: match value.member("workspace") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "workspace");
                Some(decode_agency_runs_run_workspace(member4, &at4)?)
            }
        },
    })
}

/// Writes the outcome of `agency.runs.StartRun` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_start_run(value: &crate::runs::StartRunOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::StartRunOutcome::Requested { run_requested } => {
            json::member(out, "outcome");
            json::push_text(out, "requested");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunRequested");
            json::member(out, "payload");
            encode_event_agency_runs_run_requested(run_requested, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.runs.StopRun` as JSON.
pub fn encode_command_agency_runs_stop_run(value: &crate::runs::StopRun, out: &mut String) {
    out.push('{');
    json::member(out, "run_id");
    encode_agency_runs_run_id(&value.run_id, out);
    json::member(out, "reason");
    json::push_text(out, &value.reason);
    out.push('}');
}

/// Reads the input of `agency.runs.StopRun` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_runs_stop_run(value: &json::Value, at: &str) -> Result<crate::runs::StopRun, json::DecodeError> {
    Ok(crate::runs::StopRun {
        run_id: {
            let at0 = json::nested(at, "run_id");
            let member0 = json::member_at(value, at, "run_id")?;
            decode_agency_runs_run_id(member0, &at0)?
        },
        reason: {
            let at1 = json::nested(at, "reason");
            let member1 = json::member_at(value, at, "reason")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
    })
}

/// Writes the outcome of `agency.runs.StopRun` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_runs_stop_run(value: &crate::runs::StopRunOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::runs::StopRunOutcome::NoSuchRun { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-run");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.NotFound");
            json::member(out, "payload");
            encode_error_agency_runs_not_found(error, out);
            out.push('}');
        }
        crate::runs::StopRunOutcome::Stopped { run_stopped } => {
            json::member(out, "outcome");
            json::push_text(out, "stopped");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.runs.RunStopped");
            json::member(out, "payload");
            encode_event_agency_runs_run_stopped(run_stopped, out);
            out.push('}');
            out.push(']');
        }
        crate::runs::StopRunOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.runs.StateConflict");
            json::member(out, "payload");
            encode_error_agency_runs_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.service.CloseSession` as JSON.
pub fn encode_command_agency_service_close_session(value: &crate::service::CloseSession, out: &mut String) {
    out.push('{');
    json::member(out, "session_id");
    encode_agency_service_session_id(&value.session_id, out);
    out.push('}');
}

/// Reads the input of `agency.service.CloseSession` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_service_close_session(value: &json::Value, at: &str) -> Result<crate::service::CloseSession, json::DecodeError> {
    Ok(crate::service::CloseSession {
        session_id: {
            let at0 = json::nested(at, "session_id");
            let member0 = json::member_at(value, at, "session_id")?;
            decode_agency_service_session_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.service.CloseSession` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_service_close_session(value: &crate::service::CloseSessionOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::service::CloseSessionOutcome::NoSuchSession { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-session");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.service.NotFound");
            json::member(out, "payload");
            encode_error_agency_service_not_found(error, out);
            out.push('}');
        }
        crate::service::CloseSessionOutcome::Closed { session_closed } => {
            json::member(out, "outcome");
            json::push_text(out, "closed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.service.SessionClosed");
            json::member(out, "payload");
            encode_event_agency_service_session_closed(session_closed, out);
            out.push('}');
            out.push(']');
        }
        crate::service::CloseSessionOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.service.StateConflict");
            json::member(out, "payload");
            encode_error_agency_service_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.service.OpenSession` as JSON.
pub fn encode_command_agency_service_open_session(value: &crate::service::OpenSession, out: &mut String) {
    out.push('{');
    json::member(out, "principal_id");
    encode_agency_auth_principal_id(&value.principal_id, out);
    json::member(out, "kind");
    encode_agency_service_session_kind(&value.kind, out);
    if let Some(held0) = &value.run_id {
        json::member(out, "run_id");
        encode_agency_runs_run_id(&*held0, out);
    }
    json::member(out, "token_digest");
    encode_agency_shared_digest(&value.token_digest, out);
    if let Some(held0) = &value.csrf_digest {
        json::member(out, "csrf_digest");
        encode_agency_shared_digest(&*held0, out);
    }
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    out.push('}');
}

/// Reads the input of `agency.service.OpenSession` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_service_open_session(value: &json::Value, at: &str) -> Result<crate::service::OpenSession, json::DecodeError> {
    Ok(crate::service::OpenSession {
        principal_id: {
            let at0 = json::nested(at, "principal_id");
            let member0 = json::member_at(value, at, "principal_id")?;
            decode_agency_auth_principal_id(member0, &at0)?
        },
        kind: {
            let at1 = json::nested(at, "kind");
            let member1 = json::member_at(value, at, "kind")?;
            decode_agency_service_session_kind(member1, &at1)?
        },
        run_id: match value.member("run_id") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "run_id");
                Some(decode_agency_runs_run_id(member2, &at2)?)
            }
        },
        token_digest: {
            let at3 = json::nested(at, "token_digest");
            let member3 = json::member_at(value, at, "token_digest")?;
            decode_agency_shared_digest(member3, &at3)?
        },
        csrf_digest: match value.member("csrf_digest") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "csrf_digest");
                Some(decode_agency_shared_digest(member4, &at4)?)
            }
        },
        expires_at: match value.member("expires_at") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "expires_at");
                Some(crate::primitives::Timestamp(json::text_at(member5, &at5, "an RFC 3339 instant")?.to_owned()))
            }
        },
    })
}

/// Writes the outcome of `agency.service.OpenSession` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_service_open_session(value: &crate::service::OpenSessionOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::service::OpenSessionOutcome::Opened { session_opened } => {
            json::member(out, "outcome");
            json::push_text(out, "opened");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.service.SessionOpened");
            json::member(out, "payload");
            encode_event_agency_service_session_opened(session_opened, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.AddMember` as JSON.
pub fn encode_command_agency_teams_add_member(value: &crate::teams::AddMember, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.AddMember` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_add_member(value: &json::Value, at: &str) -> Result<crate::teams::AddMember, json::DecodeError> {
    Ok(crate::teams::AddMember {
        team_id: {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agency_agents_agent_id(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.teams.AddMember` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_add_member(value: &crate::teams::AddMemberOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::AddMemberOutcome::NoSuchTeam { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-team");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::AddMemberOutcome::Added { member_added } => {
            json::member(out, "outcome");
            json::push_text(out, "added");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.MemberAdded");
            json::member(out, "payload");
            encode_event_agency_teams_member_added(member_added, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.AttachQueue` as JSON.
pub fn encode_command_agency_teams_attach_queue(value: &crate::teams::AttachQueue, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.AttachQueue` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_attach_queue(value: &json::Value, at: &str) -> Result<crate::teams::AttachQueue, json::DecodeError> {
    Ok(crate::teams::AttachQueue {
        workforce_id: {
            let at0 = json::nested(at, "workforce_id");
            let member0 = json::member_at(value, at, "workforce_id")?;
            decode_agency_teams_workforce_id(member0, &at0)?
        },
        input_id: {
            let at1 = json::nested(at, "input_id");
            let member1 = json::member_at(value, at, "input_id")?;
            decode_agency_work_input_id(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.teams.AttachQueue` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_attach_queue(value: &crate::teams::AttachQueueOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::AttachQueueOutcome::NoSuchInput { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-input");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::AttachQueueOutcome::Attached { queue_attached } => {
            json::member(out, "outcome");
            json::push_text(out, "attached");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.QueueAttached");
            json::member(out, "payload");
            encode_event_agency_teams_queue_attached(queue_attached, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.DeclareSwarm` as JSON.
pub fn encode_command_agency_teams_declare_swarm(value: &crate::teams::DeclareSwarm, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "goal_ref");
    json::push_text(out, &value.goal_ref);
    json::member(out, "coordinator_agent_id");
    encode_agency_agents_agent_id(&value.coordinator_agent_id, out);
    json::member(out, "frame_digest");
    encode_agency_shared_digest(&value.frame_digest, out);
    json::member(out, "max_workers");
    json::push_integer(out, value.max_workers);
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "work_input_id");
    encode_agency_work_input_id(&value.work_input_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.DeclareSwarm` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_declare_swarm(value: &json::Value, at: &str) -> Result<crate::teams::DeclareSwarm, json::DecodeError> {
    Ok(crate::teams::DeclareSwarm {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        goal_ref: {
            let at1 = json::nested(at, "goal_ref");
            let member1 = json::member_at(value, at, "goal_ref")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        coordinator_agent_id: {
            let at2 = json::nested(at, "coordinator_agent_id");
            let member2 = json::member_at(value, at, "coordinator_agent_id")?;
            decode_agency_agents_agent_id(member2, &at2)?
        },
        frame_digest: {
            let at3 = json::nested(at, "frame_digest");
            let member3 = json::member_at(value, at, "frame_digest")?;
            decode_agency_shared_digest(member3, &at3)?
        },
        max_workers: {
            let at4 = json::nested(at, "max_workers");
            let member4 = json::member_at(value, at, "max_workers")?;
            json::integer_at(member4, &at4, "an integer")?
        },
        team_id: {
            let at5 = json::nested(at, "team_id");
            let member5 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member5, &at5)?
        },
        work_input_id: {
            let at6 = json::nested(at, "work_input_id");
            let member6 = json::member_at(value, at, "work_input_id")?;
            decode_agency_work_input_id(member6, &at6)?
        },
    })
}

/// Writes the outcome of `agency.teams.DeclareSwarm` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_declare_swarm(value: &crate::teams::DeclareSwarmOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::DeclareSwarmOutcome::Declared { swarm_declared } => {
            json::member(out, "outcome");
            json::push_text(out, "declared");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.SwarmDeclared");
            json::member(out, "payload");
            encode_event_agency_teams_swarm_declared(swarm_declared, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.DefineWorkforce` as JSON.
pub fn encode_command_agency_teams_define_workforce(value: &crate::teams::DefineWorkforce, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "capacity");
    encode_agency_teams_capacity(&value.capacity, out);
    out.push('}');
}

/// Reads the input of `agency.teams.DefineWorkforce` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_define_workforce(value: &json::Value, at: &str) -> Result<crate::teams::DefineWorkforce, json::DecodeError> {
    Ok(crate::teams::DefineWorkforce {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        name: {
            let at1 = json::nested(at, "name");
            let member1 = json::member_at(value, at, "name")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        capacity: {
            let at2 = json::nested(at, "capacity");
            let member2 = json::member_at(value, at, "capacity")?;
            decode_agency_teams_capacity(member2, &at2)?
        },
    })
}

/// Writes the outcome of `agency.teams.DefineWorkforce` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_define_workforce(value: &crate::teams::DefineWorkforceOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::DefineWorkforceOutcome::Defined { workforce_defined } => {
            json::member(out, "outcome");
            json::push_text(out, "defined");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.WorkforceDefined");
            json::member(out, "payload");
            encode_event_agency_teams_workforce_defined(workforce_defined, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.DisbandTeam` as JSON.
pub fn encode_command_agency_teams_disband_team(value: &crate::teams::DisbandTeam, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.DisbandTeam` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_disband_team(value: &json::Value, at: &str) -> Result<crate::teams::DisbandTeam, json::DecodeError> {
    Ok(crate::teams::DisbandTeam {
        team_id: {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.teams.DisbandTeam` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_disband_team(value: &crate::teams::DisbandTeamOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::DisbandTeamOutcome::NoSuchTeam { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-team");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::DisbandTeamOutcome::Disbanded { team_disbanded } => {
            json::member(out, "outcome");
            json::push_text(out, "disbanded");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.TeamDisbanded");
            json::member(out, "payload");
            encode_event_agency_teams_team_disbanded(team_disbanded, out);
            out.push('}');
            out.push(']');
        }
        crate::teams::DisbandTeamOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.StateConflict");
            json::member(out, "payload");
            encode_error_agency_teams_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.FormTeam` as JSON.
pub fn encode_command_agency_teams_form_team(value: &crate::teams::FormTeam, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "lead_agent_id");
    encode_agency_agents_agent_id(&value.lead_agent_id, out);
    json::member(out, "members");
    out.push('[');
    for (index0, item0) in value.members.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_agents_agent_id(&*item0, out);
    }
    out.push(']');
    json::member(out, "broker_ref");
    json::push_text(out, &value.broker_ref);
    out.push('}');
}

/// Reads the input of `agency.teams.FormTeam` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_form_team(value: &json::Value, at: &str) -> Result<crate::teams::FormTeam, json::DecodeError> {
    Ok(crate::teams::FormTeam {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        name: {
            let at1 = json::nested(at, "name");
            let member1 = json::member_at(value, at, "name")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        lead_agent_id: {
            let at2 = json::nested(at, "lead_agent_id");
            let member2 = json::member_at(value, at, "lead_agent_id")?;
            decode_agency_agents_agent_id(member2, &at2)?
        },
        members: {
            let at3 = json::nested(at, "members");
            let member3 = json::member_at(value, at, "members")?;
            {
                let mut items3 = Vec::new();
                for (index3, element3) in json::items_at(member3, &at3, "an array")?.iter().enumerate() {
                    let nested3 = json::nested(&at3, &index3.to_string());
                    items3.push(decode_agency_agents_agent_id(element3, &nested3)?);
                }
                items3
            }
        },
        broker_ref: {
            let at4 = json::nested(at, "broker_ref");
            let member4 = json::member_at(value, at, "broker_ref")?;
            json::text_at(member4, &at4, "a string")?.to_owned()
        },
    })
}

/// Writes the outcome of `agency.teams.FormTeam` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_form_team(value: &crate::teams::FormTeamOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::FormTeamOutcome::Formed { team_formed } => {
            json::member(out, "outcome");
            json::push_text(out, "formed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.TeamFormed");
            json::member(out, "payload");
            encode_event_agency_teams_team_formed(team_formed, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.RemoveMember` as JSON.
pub fn encode_command_agency_teams_remove_member(value: &crate::teams::RemoveMember, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.RemoveMember` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_remove_member(value: &json::Value, at: &str) -> Result<crate::teams::RemoveMember, json::DecodeError> {
    Ok(crate::teams::RemoveMember {
        team_id: {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agency_agents_agent_id(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.teams.RemoveMember` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_remove_member(value: &crate::teams::RemoveMemberOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::RemoveMemberOutcome::NoSuchTeam { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-team");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::RemoveMemberOutcome::Removed { member_removed } => {
            json::member(out, "outcome");
            json::push_text(out, "removed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.MemberRemoved");
            json::member(out, "payload");
            encode_event_agency_teams_member_removed(member_removed, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.Report` as JSON.
pub fn encode_command_agency_teams_report(value: &crate::teams::Report, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "from_agent_id");
    encode_agency_agents_agent_id(&value.from_agent_id, out);
    json::member(out, "status");
    json::push_text(out, &value.status);
    json::member(out, "line");
    json::push_text(out, &value.line);
    if let Some(held0) = &value.to {
        json::member(out, "to");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads the input of `agency.teams.Report` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_report(value: &json::Value, at: &str) -> Result<crate::teams::Report, json::DecodeError> {
    Ok(crate::teams::Report {
        team_id: {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
        from_agent_id: {
            let at1 = json::nested(at, "from_agent_id");
            let member1 = json::member_at(value, at, "from_agent_id")?;
            decode_agency_agents_agent_id(member1, &at1)?
        },
        status: {
            let at2 = json::nested(at, "status");
            let member2 = json::member_at(value, at, "status")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        line: {
            let at3 = json::nested(at, "line");
            let member3 = json::member_at(value, at, "line")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        to: match value.member("to") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "to");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
    })
}

/// Writes the outcome of `agency.teams.Report` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_report(value: &crate::teams::ReportOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::ReportOutcome::NoSuchTeam { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-team");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::ReportOutcome::NotOneLevelUp { error } => {
            json::member(out, "outcome");
            json::push_text(out, "not-one-level-up");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotOneLevelUp");
            json::member(out, "payload");
            encode_error_agency_teams_not_one_level_up(error, out);
            out.push('}');
        }
        crate::teams::ReportOutcome::Reported { report_sent } => {
            json::member(out, "outcome");
            json::push_text(out, "reported");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.ReportSent");
            json::member(out, "payload");
            encode_event_agency_teams_report_sent(report_sent, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.RetireWorkforce` as JSON.
pub fn encode_command_agency_teams_retire_workforce(value: &crate::teams::RetireWorkforce, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.RetireWorkforce` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_retire_workforce(value: &json::Value, at: &str) -> Result<crate::teams::RetireWorkforce, json::DecodeError> {
    Ok(crate::teams::RetireWorkforce {
        workforce_id: {
            let at0 = json::nested(at, "workforce_id");
            let member0 = json::member_at(value, at, "workforce_id")?;
            decode_agency_teams_workforce_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.teams.RetireWorkforce` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_retire_workforce(value: &crate::teams::RetireWorkforceOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::RetireWorkforceOutcome::NoSuchWorkforce { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-workforce");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::RetireWorkforceOutcome::Retired { workforce_retired } => {
            json::member(out, "outcome");
            json::push_text(out, "retired");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.WorkforceRetired");
            json::member(out, "payload");
            encode_event_agency_teams_workforce_retired(workforce_retired, out);
            out.push('}');
            out.push(']');
        }
        crate::teams::RetireWorkforceOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.StateConflict");
            json::member(out, "payload");
            encode_error_agency_teams_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.Send` as JSON.
pub fn encode_command_agency_teams_send(value: &crate::teams::Send, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "from_agent_id");
    encode_agency_agents_agent_id(&value.from_agent_id, out);
    json::member(out, "to");
    json::push_text(out, &value.to);
    json::member(out, "text");
    json::push_text(out, &value.text);
    out.push('}');
}

/// Reads the input of `agency.teams.Send` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_send(value: &json::Value, at: &str) -> Result<crate::teams::Send, json::DecodeError> {
    Ok(crate::teams::Send {
        team_id: {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
        from_agent_id: {
            let at1 = json::nested(at, "from_agent_id");
            let member1 = json::member_at(value, at, "from_agent_id")?;
            decode_agency_agents_agent_id(member1, &at1)?
        },
        to: {
            let at2 = json::nested(at, "to");
            let member2 = json::member_at(value, at, "to")?;
            json::text_at(member2, &at2, "a string")?.to_owned()
        },
        text: {
            let at3 = json::nested(at, "text");
            let member3 = json::member_at(value, at, "text")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
    })
}

/// Writes the outcome of `agency.teams.Send` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_send(value: &crate::teams::SendOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::SendOutcome::NoSuchTeam { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-team");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::SendOutcome::ForeignInstance { error } => {
            json::member(out, "outcome");
            json::push_text(out, "foreign-instance");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.ForeignInstance");
            json::member(out, "payload");
            encode_error_agency_teams_foreign_instance(error, out);
            out.push('}');
        }
        crate::teams::SendOutcome::OutsideTeam { error } => {
            json::member(out, "outcome");
            json::push_text(out, "outside-team");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.OutsideTeam");
            json::member(out, "payload");
            encode_error_agency_teams_outside_team(error, out);
            out.push('}');
        }
        crate::teams::SendOutcome::Sent { message_sent } => {
            json::member(out, "outcome");
            json::push_text(out, "sent");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.MessageSent");
            json::member(out, "payload");
            encode_event_agency_teams_message_sent(message_sent, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.SetLead` as JSON.
pub fn encode_command_agency_teams_set_lead(value: &crate::teams::SetLead, out: &mut String) {
    out.push('{');
    json::member(out, "team_id");
    encode_agency_teams_team_id(&value.team_id, out);
    json::member(out, "lead_agent_id");
    encode_agency_agents_agent_id(&value.lead_agent_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.SetLead` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_set_lead(value: &json::Value, at: &str) -> Result<crate::teams::SetLead, json::DecodeError> {
    Ok(crate::teams::SetLead {
        team_id: {
            let at0 = json::nested(at, "team_id");
            let member0 = json::member_at(value, at, "team_id")?;
            decode_agency_teams_team_id(member0, &at0)?
        },
        lead_agent_id: {
            let at1 = json::nested(at, "lead_agent_id");
            let member1 = json::member_at(value, at, "lead_agent_id")?;
            decode_agency_agents_agent_id(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.teams.SetLead` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_set_lead(value: &crate::teams::SetLeadOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::SetLeadOutcome::NoSuchTeam { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-team");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::SetLeadOutcome::Set { lead_set } => {
            json::member(out, "outcome");
            json::push_text(out, "set");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.LeadSet");
            json::member(out, "payload");
            encode_event_agency_teams_lead_set(lead_set, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.StaffWorkforce` as JSON.
pub fn encode_command_agency_teams_staff_workforce(value: &crate::teams::StaffWorkforce, out: &mut String) {
    out.push('{');
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    json::member(out, "teams");
    out.push('[');
    for (index0, item0) in value.teams.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_teams_team_id(&*item0, out);
    }
    out.push(']');
    json::member(out, "solo_agents");
    out.push('[');
    for (index0, item0) in value.solo_agents.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_agents_agent_id(&*item0, out);
    }
    out.push(']');
    out.push('}');
}

/// Reads the input of `agency.teams.StaffWorkforce` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_staff_workforce(value: &json::Value, at: &str) -> Result<crate::teams::StaffWorkforce, json::DecodeError> {
    Ok(crate::teams::StaffWorkforce {
        workforce_id: {
            let at0 = json::nested(at, "workforce_id");
            let member0 = json::member_at(value, at, "workforce_id")?;
            decode_agency_teams_workforce_id(member0, &at0)?
        },
        teams: {
            let at1 = json::nested(at, "teams");
            let member1 = json::member_at(value, at, "teams")?;
            {
                let mut items1 = Vec::new();
                for (index1, element1) in json::items_at(member1, &at1, "an array")?.iter().enumerate() {
                    let nested1 = json::nested(&at1, &index1.to_string());
                    items1.push(decode_agency_teams_team_id(element1, &nested1)?);
                }
                items1
            }
        },
        solo_agents: {
            let at2 = json::nested(at, "solo_agents");
            let member2 = json::member_at(value, at, "solo_agents")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agency_agents_agent_id(element2, &nested2)?);
                }
                items2
            }
        },
    })
}

/// Writes the outcome of `agency.teams.StaffWorkforce` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_staff_workforce(value: &crate::teams::StaffWorkforceOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::StaffWorkforceOutcome::NoSuchWorkforce { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-workforce");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::StaffWorkforceOutcome::Staffed { workforce_staffed } => {
            json::member(out, "outcome");
            json::push_text(out, "staffed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.WorkforceStaffed");
            json::member(out, "payload");
            encode_event_agency_teams_workforce_staffed(workforce_staffed, out);
            out.push('}');
            out.push(']');
        }
        crate::teams::StaffWorkforceOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.StateConflict");
            json::member(out, "payload");
            encode_error_agency_teams_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.teams.StopSwarm` as JSON.
pub fn encode_command_agency_teams_stop_swarm(value: &crate::teams::StopSwarm, out: &mut String) {
    out.push('{');
    json::member(out, "swarm_id");
    encode_agency_teams_swarm_id(&value.swarm_id, out);
    out.push('}');
}

/// Reads the input of `agency.teams.StopSwarm` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_teams_stop_swarm(value: &json::Value, at: &str) -> Result<crate::teams::StopSwarm, json::DecodeError> {
    Ok(crate::teams::StopSwarm {
        swarm_id: {
            let at0 = json::nested(at, "swarm_id");
            let member0 = json::member_at(value, at, "swarm_id")?;
            decode_agency_teams_swarm_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.teams.StopSwarm` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_teams_stop_swarm(value: &crate::teams::StopSwarmOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::teams::StopSwarmOutcome::NoSuchSwarm { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-swarm");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.NotFound");
            json::member(out, "payload");
            encode_error_agency_teams_not_found(error, out);
            out.push('}');
        }
        crate::teams::StopSwarmOutcome::Stopped { swarm_stopped } => {
            json::member(out, "outcome");
            json::push_text(out, "stopped");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.teams.SwarmStopped");
            json::member(out, "payload");
            encode_event_agency_teams_swarm_stopped(swarm_stopped, out);
            out.push('}');
            out.push(']');
        }
        crate::teams::StopSwarmOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.teams.StateConflict");
            json::member(out, "payload");
            encode_error_agency_teams_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.tools.BindTool` as JSON.
pub fn encode_command_agency_tools_bind_tool(value: &crate::tools::BindTool, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    if let Some(held0) = &value.definition_id {
        json::member(out, "definition_id");
        encode_agency_agents_definition_id(&*held0, out);
    }
    if let Some(held0) = &value.team_id {
        json::member(out, "team_id");
        encode_agency_teams_team_id(&*held0, out);
    }
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    json::member(out, "verbs");
    out.push('[');
    for (index0, item0) in value.verbs.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_shared_verb(&*item0, out);
    }
    out.push(']');
    json::member(out, "scope");
    encode_agency_tools_binding_scope(&value.scope, out);
    json::member(out, "max_output_bytes");
    json::push_integer(out, value.max_output_bytes);
    if let Some(held0) = &value.expires_at {
        json::member(out, "expires_at");
        json::push_text(out, &(*held0).0);
    }
    if let Some(held0) = &value.credential_id {
        json::member(out, "credential_id");
        encode_agency_auth_credential_id(&*held0, out);
    }
    out.push('}');
}

/// Reads the input of `agency.tools.BindTool` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_tools_bind_tool(value: &json::Value, at: &str) -> Result<crate::tools::BindTool, json::DecodeError> {
    Ok(crate::tools::BindTool {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        definition_id: match value.member("definition_id") {
            None | Some(json::Value::Null) => None,
            Some(member1) => {
                let at1 = json::nested(at, "definition_id");
                Some(decode_agency_agents_definition_id(member1, &at1)?)
            }
        },
        team_id: match value.member("team_id") {
            None | Some(json::Value::Null) => None,
            Some(member2) => {
                let at2 = json::nested(at, "team_id");
                Some(decode_agency_teams_team_id(member2, &at2)?)
            }
        },
        tool_id: {
            let at3 = json::nested(at, "tool_id");
            let member3 = json::member_at(value, at, "tool_id")?;
            decode_agency_tools_tool_id(member3, &at3)?
        },
        verbs: {
            let at4 = json::nested(at, "verbs");
            let member4 = json::member_at(value, at, "verbs")?;
            {
                let mut items4 = Vec::new();
                for (index4, element4) in json::items_at(member4, &at4, "an array")?.iter().enumerate() {
                    let nested4 = json::nested(&at4, &index4.to_string());
                    items4.push(decode_agency_shared_verb(element4, &nested4)?);
                }
                items4
            }
        },
        scope: {
            let at5 = json::nested(at, "scope");
            let member5 = json::member_at(value, at, "scope")?;
            decode_agency_tools_binding_scope(member5, &at5)?
        },
        max_output_bytes: {
            let at6 = json::nested(at, "max_output_bytes");
            let member6 = json::member_at(value, at, "max_output_bytes")?;
            json::integer_at(member6, &at6, "an integer")?
        },
        expires_at: match value.member("expires_at") {
            None | Some(json::Value::Null) => None,
            Some(member7) => {
                let at7 = json::nested(at, "expires_at");
                Some(crate::primitives::Timestamp(json::text_at(member7, &at7, "an RFC 3339 instant")?.to_owned()))
            }
        },
        credential_id: match value.member("credential_id") {
            None | Some(json::Value::Null) => None,
            Some(member8) => {
                let at8 = json::nested(at, "credential_id");
                Some(decode_agency_auth_credential_id(member8, &at8)?)
            }
        },
    })
}

/// Writes the outcome of `agency.tools.BindTool` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_tools_bind_tool(value: &crate::tools::BindToolOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::tools::BindToolOutcome::BoundToDefinition { tool_bound } => {
            json::member(out, "outcome");
            json::push_text(out, "bound-to-definition");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.tools.ToolBound");
            json::member(out, "payload");
            encode_event_agency_tools_tool_bound(tool_bound, out);
            out.push('}');
            out.push(']');
        }
        crate::tools::BindToolOutcome::BoundToTeam { tool_bound } => {
            json::member(out, "outcome");
            json::push_text(out, "bound-to-team");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.tools.ToolBound");
            json::member(out, "payload");
            encode_event_agency_tools_tool_bound(tool_bound, out);
            out.push('}');
            out.push(']');
        }
        crate::tools::BindToolOutcome::TargetInvalid { error } => {
            json::member(out, "outcome");
            json::push_text(out, "target-invalid");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.tools.BindingTargetInvalid");
            json::member(out, "payload");
            encode_error_agency_tools_binding_target_invalid(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.tools.RegisterTool` as JSON.
pub fn encode_command_agency_tools_register_tool(value: &crate::tools::RegisterTool, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "contract_ref");
    encode_agency_tools_contract_ref(&value.contract_ref, out);
    json::member(out, "verbs");
    out.push('[');
    for (index0, item0) in value.verbs.iter().enumerate() {
        if index0 > 0 {
            out.push(',');
        }
        encode_agency_shared_verb(&*item0, out);
    }
    out.push(']');
    json::member(out, "risk");
    encode_agency_tools_risk_envelope(&value.risk, out);
    if let Some(held0) = &value.implementation_ref {
        json::member(out, "implementation_ref");
        json::push_text(out, &*held0);
    }
    if let Some(held0) = &value.connector_instance_id {
        json::member(out, "connector_instance_id");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads the input of `agency.tools.RegisterTool` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_tools_register_tool(value: &json::Value, at: &str) -> Result<crate::tools::RegisterTool, json::DecodeError> {
    Ok(crate::tools::RegisterTool {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        contract_ref: {
            let at1 = json::nested(at, "contract_ref");
            let member1 = json::member_at(value, at, "contract_ref")?;
            decode_agency_tools_contract_ref(member1, &at1)?
        },
        verbs: {
            let at2 = json::nested(at, "verbs");
            let member2 = json::member_at(value, at, "verbs")?;
            {
                let mut items2 = Vec::new();
                for (index2, element2) in json::items_at(member2, &at2, "an array")?.iter().enumerate() {
                    let nested2 = json::nested(&at2, &index2.to_string());
                    items2.push(decode_agency_shared_verb(element2, &nested2)?);
                }
                items2
            }
        },
        risk: {
            let at3 = json::nested(at, "risk");
            let member3 = json::member_at(value, at, "risk")?;
            decode_agency_tools_risk_envelope(member3, &at3)?
        },
        implementation_ref: match value.member("implementation_ref") {
            None | Some(json::Value::Null) => None,
            Some(member4) => {
                let at4 = json::nested(at, "implementation_ref");
                Some(json::text_at(member4, &at4, "a string")?.to_owned())
            }
        },
        connector_instance_id: match value.member("connector_instance_id") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "connector_instance_id");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
    })
}

/// Writes the outcome of `agency.tools.RegisterTool` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_tools_register_tool(value: &crate::tools::RegisterToolOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::tools::RegisterToolOutcome::Registered { tool_registered } => {
            json::member(out, "outcome");
            json::push_text(out, "registered");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.tools.ToolRegistered");
            json::member(out, "payload");
            encode_event_agency_tools_tool_registered(tool_registered, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.tools.RevokeBinding` as JSON.
pub fn encode_command_agency_tools_revoke_binding(value: &crate::tools::RevokeBinding, out: &mut String) {
    out.push('{');
    json::member(out, "binding_id");
    encode_agency_tools_binding_id(&value.binding_id, out);
    out.push('}');
}

/// Reads the input of `agency.tools.RevokeBinding` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_tools_revoke_binding(value: &json::Value, at: &str) -> Result<crate::tools::RevokeBinding, json::DecodeError> {
    Ok(crate::tools::RevokeBinding {
        binding_id: {
            let at0 = json::nested(at, "binding_id");
            let member0 = json::member_at(value, at, "binding_id")?;
            decode_agency_tools_binding_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.tools.RevokeBinding` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_tools_revoke_binding(value: &crate::tools::RevokeBindingOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::tools::RevokeBindingOutcome::NoSuchBinding { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-binding");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.tools.NotFound");
            json::member(out, "payload");
            encode_error_agency_tools_not_found(error, out);
            out.push('}');
        }
        crate::tools::RevokeBindingOutcome::Revoked { binding_revoked } => {
            json::member(out, "outcome");
            json::push_text(out, "revoked");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.tools.BindingRevoked");
            json::member(out, "payload");
            encode_event_agency_tools_binding_revoked(binding_revoked, out);
            out.push('}');
            out.push(']');
        }
        crate::tools::RevokeBindingOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.tools.StateConflict");
            json::member(out, "payload");
            encode_error_agency_tools_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.tools.WithdrawTool` as JSON.
pub fn encode_command_agency_tools_withdraw_tool(value: &crate::tools::WithdrawTool, out: &mut String) {
    out.push('{');
    json::member(out, "tool_id");
    encode_agency_tools_tool_id(&value.tool_id, out);
    out.push('}');
}

/// Reads the input of `agency.tools.WithdrawTool` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_tools_withdraw_tool(value: &json::Value, at: &str) -> Result<crate::tools::WithdrawTool, json::DecodeError> {
    Ok(crate::tools::WithdrawTool {
        tool_id: {
            let at0 = json::nested(at, "tool_id");
            let member0 = json::member_at(value, at, "tool_id")?;
            decode_agency_tools_tool_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.tools.WithdrawTool` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_tools_withdraw_tool(value: &crate::tools::WithdrawToolOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::tools::WithdrawToolOutcome::NoSuchTool { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-tool");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.tools.NotFound");
            json::member(out, "payload");
            encode_error_agency_tools_not_found(error, out);
            out.push('}');
        }
        crate::tools::WithdrawToolOutcome::Withdrawn { tool_withdrawn } => {
            json::member(out, "outcome");
            json::push_text(out, "withdrawn");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.tools.ToolWithdrawn");
            json::member(out, "payload");
            encode_event_agency_tools_tool_withdrawn(tool_withdrawn, out);
            out.push('}');
            out.push(']');
        }
        crate::tools::WithdrawToolOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.tools.StateConflict");
            json::member(out, "payload");
            encode_error_agency_tools_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.AbandonWorkItem` as JSON.
pub fn encode_command_agency_work_abandon_work_item(value: &crate::work::AbandonWorkItem, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    out.push('}');
}

/// Reads the input of `agency.work.AbandonWorkItem` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_abandon_work_item(value: &json::Value, at: &str) -> Result<crate::work::AbandonWorkItem, json::DecodeError> {
    Ok(crate::work::AbandonWorkItem {
        item_ref: {
            let at0 = json::nested(at, "item_ref");
            let member0 = json::member_at(value, at, "item_ref")?;
            decode_agency_work_work_item_ref(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.work.AbandonWorkItem` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_abandon_work_item(value: &crate::work::AbandonWorkItemOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::AbandonWorkItemOutcome::NoSuchItem { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-item");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.NotFound");
            json::member(out, "payload");
            encode_error_agency_work_not_found(error, out);
            out.push('}');
        }
        crate::work::AbandonWorkItemOutcome::Abandoned { work_item_abandoned } => {
            json::member(out, "outcome");
            json::push_text(out, "abandoned");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkItemAbandoned");
            json::member(out, "payload");
            encode_event_agency_work_work_item_abandoned(work_item_abandoned, out);
            out.push('}');
            out.push(']');
        }
        crate::work::AbandonWorkItemOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.StateConflict");
            json::member(out, "payload");
            encode_error_agency_work_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.AssignWorkItem` as JSON.
pub fn encode_command_agency_work_assign_work_item(value: &crate::work::AssignWorkItem, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    json::member(out, "agent_id");
    encode_agency_agents_agent_id(&value.agent_id, out);
    out.push('}');
}

/// Reads the input of `agency.work.AssignWorkItem` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_assign_work_item(value: &json::Value, at: &str) -> Result<crate::work::AssignWorkItem, json::DecodeError> {
    Ok(crate::work::AssignWorkItem {
        item_ref: {
            let at0 = json::nested(at, "item_ref");
            let member0 = json::member_at(value, at, "item_ref")?;
            decode_agency_work_work_item_ref(member0, &at0)?
        },
        agent_id: {
            let at1 = json::nested(at, "agent_id");
            let member1 = json::member_at(value, at, "agent_id")?;
            decode_agency_agents_agent_id(member1, &at1)?
        },
    })
}

/// Writes the outcome of `agency.work.AssignWorkItem` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_assign_work_item(value: &crate::work::AssignWorkItemOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::AssignWorkItemOutcome::NoSuchItem { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-item");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.NotFound");
            json::member(out, "payload");
            encode_error_agency_work_not_found(error, out);
            out.push('}');
        }
        crate::work::AssignWorkItemOutcome::Assigned { work_item_assigned } => {
            json::member(out, "outcome");
            json::push_text(out, "assigned");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkItemAssigned");
            json::member(out, "payload");
            encode_event_agency_work_work_item_assigned(work_item_assigned, out);
            out.push('}');
            out.push(']');
        }
        crate::work::AssignWorkItemOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.StateConflict");
            json::member(out, "payload");
            encode_error_agency_work_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.ConfigureWorkInput` as JSON.
pub fn encode_command_agency_work_configure_work_input(value: &crate::work::ConfigureWorkInput, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "name");
    json::push_text(out, &value.name);
    json::member(out, "source");
    encode_agency_work_work_source(&value.source, out);
    if let Some(held0) = &value.poll_interval {
        json::member(out, "poll_interval");
        json::push_text(out, &(*held0).0);
    }
    json::member(out, "workforce_id");
    encode_agency_teams_workforce_id(&value.workforce_id, out);
    if let Some(held0) = &value.transform_ref {
        json::member(out, "transform_ref");
        json::push_text(out, &*held0);
    }
    out.push('}');
}

/// Reads the input of `agency.work.ConfigureWorkInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_configure_work_input(value: &json::Value, at: &str) -> Result<crate::work::ConfigureWorkInput, json::DecodeError> {
    Ok(crate::work::ConfigureWorkInput {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        name: {
            let at1 = json::nested(at, "name");
            let member1 = json::member_at(value, at, "name")?;
            json::text_at(member1, &at1, "a string")?.to_owned()
        },
        source: {
            let at2 = json::nested(at, "source");
            let member2 = json::member_at(value, at, "source")?;
            decode_agency_work_work_source(member2, &at2)?
        },
        poll_interval: match value.member("poll_interval") {
            None | Some(json::Value::Null) => None,
            Some(member3) => {
                let at3 = json::nested(at, "poll_interval");
                Some(crate::primitives::Duration(json::text_at(member3, &at3, "an ISO 8601 duration")?.to_owned()))
            }
        },
        workforce_id: {
            let at4 = json::nested(at, "workforce_id");
            let member4 = json::member_at(value, at, "workforce_id")?;
            decode_agency_teams_workforce_id(member4, &at4)?
        },
        transform_ref: match value.member("transform_ref") {
            None | Some(json::Value::Null) => None,
            Some(member5) => {
                let at5 = json::nested(at, "transform_ref");
                Some(json::text_at(member5, &at5, "a string")?.to_owned())
            }
        },
    })
}

/// Writes the outcome of `agency.work.ConfigureWorkInput` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_configure_work_input(value: &crate::work::ConfigureWorkInputOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::ConfigureWorkInputOutcome::Configured { work_input_configured } => {
            json::member(out, "outcome");
            json::push_text(out, "configured");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkInputConfigured");
            json::member(out, "payload");
            encode_event_agency_work_work_input_configured(work_input_configured, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.PauseWorkInput` as JSON.
pub fn encode_command_agency_work_pause_work_input(value: &crate::work::PauseWorkInput, out: &mut String) {
    out.push('{');
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    out.push('}');
}

/// Reads the input of `agency.work.PauseWorkInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_pause_work_input(value: &json::Value, at: &str) -> Result<crate::work::PauseWorkInput, json::DecodeError> {
    Ok(crate::work::PauseWorkInput {
        input_id: {
            let at0 = json::nested(at, "input_id");
            let member0 = json::member_at(value, at, "input_id")?;
            decode_agency_work_input_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.work.PauseWorkInput` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_pause_work_input(value: &crate::work::PauseWorkInputOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::PauseWorkInputOutcome::NoSuchInput { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-input");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.NotFound");
            json::member(out, "payload");
            encode_error_agency_work_not_found(error, out);
            out.push('}');
        }
        crate::work::PauseWorkInputOutcome::Paused { work_input_paused } => {
            json::member(out, "outcome");
            json::push_text(out, "paused");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkInputPaused");
            json::member(out, "payload");
            encode_event_agency_work_work_input_paused(work_input_paused, out);
            out.push('}');
            out.push(']');
        }
        crate::work::PauseWorkInputOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.StateConflict");
            json::member(out, "payload");
            encode_error_agency_work_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.RemoveWorkInput` as JSON.
pub fn encode_command_agency_work_remove_work_input(value: &crate::work::RemoveWorkInput, out: &mut String) {
    out.push('{');
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    out.push('}');
}

/// Reads the input of `agency.work.RemoveWorkInput` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_remove_work_input(value: &json::Value, at: &str) -> Result<crate::work::RemoveWorkInput, json::DecodeError> {
    Ok(crate::work::RemoveWorkInput {
        input_id: {
            let at0 = json::nested(at, "input_id");
            let member0 = json::member_at(value, at, "input_id")?;
            decode_agency_work_input_id(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.work.RemoveWorkInput` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_remove_work_input(value: &crate::work::RemoveWorkInputOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::RemoveWorkInputOutcome::NoSuchInput { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-input");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.NotFound");
            json::member(out, "payload");
            encode_error_agency_work_not_found(error, out);
            out.push('}');
        }
        crate::work::RemoveWorkInputOutcome::Removed { work_input_removed } => {
            json::member(out, "outcome");
            json::push_text(out, "removed");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkInputRemoved");
            json::member(out, "payload");
            encode_event_agency_work_work_input_removed(work_input_removed, out);
            out.push('}');
            out.push(']');
        }
        crate::work::RemoveWorkInputOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.StateConflict");
            json::member(out, "payload");
            encode_error_agency_work_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.ReturnWorkItem` as JSON.
pub fn encode_command_agency_work_return_work_item(value: &crate::work::ReturnWorkItem, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    out.push('}');
}

/// Reads the input of `agency.work.ReturnWorkItem` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_return_work_item(value: &json::Value, at: &str) -> Result<crate::work::ReturnWorkItem, json::DecodeError> {
    Ok(crate::work::ReturnWorkItem {
        item_ref: {
            let at0 = json::nested(at, "item_ref");
            let member0 = json::member_at(value, at, "item_ref")?;
            decode_agency_work_work_item_ref(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.work.ReturnWorkItem` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_return_work_item(value: &crate::work::ReturnWorkItemOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::ReturnWorkItemOutcome::NoSuchItem { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-item");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.NotFound");
            json::member(out, "payload");
            encode_error_agency_work_not_found(error, out);
            out.push('}');
        }
        crate::work::ReturnWorkItemOutcome::Returned { work_item_returned } => {
            json::member(out, "outcome");
            json::push_text(out, "returned");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkItemReturned");
            json::member(out, "payload");
            encode_event_agency_work_work_item_returned(work_item_returned, out);
            out.push('}');
            out.push(']');
        }
        crate::work::ReturnWorkItemOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.StateConflict");
            json::member(out, "payload");
            encode_error_agency_work_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.SettleWorkItem` as JSON.
pub fn encode_command_agency_work_settle_work_item(value: &crate::work::SettleWorkItem, out: &mut String) {
    out.push('{');
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    out.push('}');
}

/// Reads the input of `agency.work.SettleWorkItem` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_settle_work_item(value: &json::Value, at: &str) -> Result<crate::work::SettleWorkItem, json::DecodeError> {
    Ok(crate::work::SettleWorkItem {
        item_ref: {
            let at0 = json::nested(at, "item_ref");
            let member0 = json::member_at(value, at, "item_ref")?;
            decode_agency_work_work_item_ref(member0, &at0)?
        },
    })
}

/// Writes the outcome of `agency.work.SettleWorkItem` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_settle_work_item(value: &crate::work::SettleWorkItemOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::SettleWorkItemOutcome::NoSuchItem { error } => {
            json::member(out, "outcome");
            json::push_text(out, "no-such-item");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.NotFound");
            json::member(out, "payload");
            encode_error_agency_work_not_found(error, out);
            out.push('}');
        }
        crate::work::SettleWorkItemOutcome::Settled { work_item_settled } => {
            json::member(out, "outcome");
            json::push_text(out, "settled");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkItemSettled");
            json::member(out, "payload");
            encode_event_agency_work_work_item_settled(work_item_settled, out);
            out.push('}');
            out.push(']');
        }
        crate::work::SettleWorkItemOutcome::WrongState { error } => {
            json::member(out, "outcome");
            json::push_text(out, "wrong-state");
            json::member(out, "published");
            out.push('[');
            out.push(']');
            json::member(out, "refusal");
            out.push('{');
            json::member(out, "error");
            json::push_text(out, "agency.work.StateConflict");
            json::member(out, "payload");
            encode_error_agency_work_state_conflict(error, out);
            out.push('}');
        }
    }
    out.push('}');
}

/// Writes the input of `agency.work.SubmitWorkItem` as JSON.
pub fn encode_command_agency_work_submit_work_item(value: &crate::work::SubmitWorkItem, out: &mut String) {
    out.push('{');
    json::member(out, "tenant_id");
    encode_agency_auth_tenant_id(&value.tenant_id, out);
    json::member(out, "item_ref");
    encode_agency_work_work_item_ref(&value.item_ref, out);
    json::member(out, "input_id");
    encode_agency_work_input_id(&value.input_id, out);
    json::member(out, "title");
    json::push_text(out, &value.title);
    json::member(out, "payload_digest");
    encode_agency_shared_digest(&value.payload_digest, out);
    out.push('}');
}

/// Reads the input of `agency.work.SubmitWorkItem` from JSON.
///
/// # Errors
///
/// [`json::DecodeError`] naming the path and what the declaration says belongs there.
pub fn decode_command_agency_work_submit_work_item(value: &json::Value, at: &str) -> Result<crate::work::SubmitWorkItem, json::DecodeError> {
    Ok(crate::work::SubmitWorkItem {
        tenant_id: {
            let at0 = json::nested(at, "tenant_id");
            let member0 = json::member_at(value, at, "tenant_id")?;
            decode_agency_auth_tenant_id(member0, &at0)?
        },
        item_ref: {
            let at1 = json::nested(at, "item_ref");
            let member1 = json::member_at(value, at, "item_ref")?;
            decode_agency_work_work_item_ref(member1, &at1)?
        },
        input_id: {
            let at2 = json::nested(at, "input_id");
            let member2 = json::member_at(value, at, "input_id")?;
            decode_agency_work_input_id(member2, &at2)?
        },
        title: {
            let at3 = json::nested(at, "title");
            let member3 = json::member_at(value, at, "title")?;
            json::text_at(member3, &at3, "a string")?.to_owned()
        },
        payload_digest: {
            let at4 = json::nested(at, "payload_digest");
            let member4 = json::member_at(value, at, "payload_digest")?;
            decode_agency_shared_digest(member4, &at4)?
        },
    })
}

/// Writes the outcome of `agency.work.SubmitWorkItem` as JSON: the branch taken, what it published, and the declared
/// refusal it carries where it carries one.
pub fn encode_outcome_agency_work_submit_work_item(value: &crate::work::SubmitWorkItemOutcome, out: &mut String) {
    out.push('{');
    match value {
        crate::work::SubmitWorkItemOutcome::Received { work_item_submitted } => {
            json::member(out, "outcome");
            json::push_text(out, "received");
            json::member(out, "published");
            out.push('[');
            out.push('{');
            json::member(out, "event");
            json::push_text(out, "agency.work.WorkItemSubmitted");
            json::member(out, "payload");
            encode_event_agency_work_work_item_submitted(work_item_submitted, out);
            out.push('}');
            out.push(']');
        }
    }
    out.push('}');
}

/// Writes any event on the system's log as JSON: its qualified name and its payload,
/// `{"event": …, "payload": {…}}`, the envelope a command's answer lists it in.
pub fn encode_system_event(value: &crate::system::SystemEvent) -> String {
    let mut out = String::from("{");
    json::member(&mut out, "event");
    json::push_text(&mut out, value.name());
    json::member(&mut out, "payload");
    match value {
        crate::system::SystemEvent::AgentDefinitionCreated(event) => encode_event_agency_agents_agent_definition_created(event, &mut out),
        crate::system::SystemEvent::AgentDefinitionDeprecated(event) => encode_event_agency_agents_agent_definition_deprecated(event, &mut out),
        crate::system::SystemEvent::AgentDefinitionPublished(event) => encode_event_agency_agents_agent_definition_published(event, &mut out),
        crate::system::SystemEvent::AgentProvisioned(event) => encode_event_agency_agents_agent_provisioned(event, &mut out),
        crate::system::SystemEvent::AgentRetired(event) => encode_event_agency_agents_agent_retired(event, &mut out),
        crate::system::SystemEvent::AgentSpecChecked(event) => encode_event_agency_agents_agent_spec_checked(event, &mut out),
        crate::system::SystemEvent::AgentSpecRefused(event) => encode_event_agency_agents_agent_spec_refused(event, &mut out),
        crate::system::SystemEvent::CredentialGranted(event) => encode_event_agency_auth_credential_granted(event, &mut out),
        crate::system::SystemEvent::CredentialRefreshed(event) => encode_event_agency_auth_credential_refreshed(event, &mut out),
        crate::system::SystemEvent::CredentialRevoked(event) => encode_event_agency_auth_credential_revoked(event, &mut out),
        crate::system::SystemEvent::CredentialStored(event) => encode_event_agency_auth_credential_stored(event, &mut out),
        crate::system::SystemEvent::GrantRevoked(event) => encode_event_agency_auth_grant_revoked(event, &mut out),
        crate::system::SystemEvent::PolicyActivated(event) => encode_event_agency_auth_policy_activated(event, &mut out),
        crate::system::SystemEvent::PolicyCreated(event) => encode_event_agency_auth_policy_created(event, &mut out),
        crate::system::SystemEvent::PolicySuperseded(event) => encode_event_agency_auth_policy_superseded(event, &mut out),
        crate::system::SystemEvent::PrincipalDisabled(event) => encode_event_agency_auth_principal_disabled(event, &mut out),
        crate::system::SystemEvent::PrincipalInvited(event) => encode_event_agency_auth_principal_invited(event, &mut out),
        crate::system::SystemEvent::RoleAssigned(event) => encode_event_agency_auth_role_assigned(event, &mut out),
        crate::system::SystemEvent::TenantCreated(event) => encode_event_agency_auth_tenant_created(event, &mut out),
        crate::system::SystemEvent::RunAdmitted(event) => encode_event_agency_runs_run_admitted(event, &mut out),
        crate::system::SystemEvent::RunApprovalDenied(event) => encode_event_agency_runs_run_approval_denied(event, &mut out),
        crate::system::SystemEvent::RunApprovalGranted(event) => encode_event_agency_runs_run_approval_granted(event, &mut out),
        crate::system::SystemEvent::RunApprovalRequested(event) => encode_event_agency_runs_run_approval_requested(event, &mut out),
        crate::system::SystemEvent::RunBudgetSet(event) => encode_event_agency_runs_run_budget_set(event, &mut out),
        crate::system::SystemEvent::RunFailed(event) => encode_event_agency_runs_run_failed(event, &mut out),
        crate::system::SystemEvent::RunProposed(event) => encode_event_agency_runs_run_proposed(event, &mut out),
        crate::system::SystemEvent::RunRequested(event) => encode_event_agency_runs_run_requested(event, &mut out),
        crate::system::SystemEvent::RunStarted(event) => encode_event_agency_runs_run_started(event, &mut out),
        crate::system::SystemEvent::RunStopped(event) => encode_event_agency_runs_run_stopped(event, &mut out),
        crate::system::SystemEvent::SessionClosed(event) => encode_event_agency_service_session_closed(event, &mut out),
        crate::system::SystemEvent::SessionOpened(event) => encode_event_agency_service_session_opened(event, &mut out),
        crate::system::SystemEvent::LeadSet(event) => encode_event_agency_teams_lead_set(event, &mut out),
        crate::system::SystemEvent::MemberAdded(event) => encode_event_agency_teams_member_added(event, &mut out),
        crate::system::SystemEvent::MemberRemoved(event) => encode_event_agency_teams_member_removed(event, &mut out),
        crate::system::SystemEvent::MessageSent(event) => encode_event_agency_teams_message_sent(event, &mut out),
        crate::system::SystemEvent::QueueAttached(event) => encode_event_agency_teams_queue_attached(event, &mut out),
        crate::system::SystemEvent::ReportSent(event) => encode_event_agency_teams_report_sent(event, &mut out),
        crate::system::SystemEvent::SwarmDeclared(event) => encode_event_agency_teams_swarm_declared(event, &mut out),
        crate::system::SystemEvent::SwarmStopped(event) => encode_event_agency_teams_swarm_stopped(event, &mut out),
        crate::system::SystemEvent::TeamDisbanded(event) => encode_event_agency_teams_team_disbanded(event, &mut out),
        crate::system::SystemEvent::TeamFormed(event) => encode_event_agency_teams_team_formed(event, &mut out),
        crate::system::SystemEvent::WorkforceDefined(event) => encode_event_agency_teams_workforce_defined(event, &mut out),
        crate::system::SystemEvent::WorkforceRetired(event) => encode_event_agency_teams_workforce_retired(event, &mut out),
        crate::system::SystemEvent::WorkforceStaffed(event) => encode_event_agency_teams_workforce_staffed(event, &mut out),
        crate::system::SystemEvent::BindingRevoked(event) => encode_event_agency_tools_binding_revoked(event, &mut out),
        crate::system::SystemEvent::ToolBound(event) => encode_event_agency_tools_tool_bound(event, &mut out),
        crate::system::SystemEvent::ToolRegistered(event) => encode_event_agency_tools_tool_registered(event, &mut out),
        crate::system::SystemEvent::ToolWithdrawn(event) => encode_event_agency_tools_tool_withdrawn(event, &mut out),
        crate::system::SystemEvent::WorkInputConfigured(event) => encode_event_agency_work_work_input_configured(event, &mut out),
        crate::system::SystemEvent::WorkInputPaused(event) => encode_event_agency_work_work_input_paused(event, &mut out),
        crate::system::SystemEvent::WorkInputRemoved(event) => encode_event_agency_work_work_input_removed(event, &mut out),
        crate::system::SystemEvent::WorkItemAbandoned(event) => encode_event_agency_work_work_item_abandoned(event, &mut out),
        crate::system::SystemEvent::WorkItemAssigned(event) => encode_event_agency_work_work_item_assigned(event, &mut out),
        crate::system::SystemEvent::WorkItemReturned(event) => encode_event_agency_work_work_item_returned(event, &mut out),
        crate::system::SystemEvent::WorkItemSettled(event) => encode_event_agency_work_work_item_settled(event, &mut out),
        crate::system::SystemEvent::WorkItemSubmitted(event) => encode_event_agency_work_work_item_submitted(event, &mut out),
    }
    out.push('}');
    out
}
