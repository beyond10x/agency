// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Ephemeral stores for the generated network entry points; no durability.

/// Ephemeral storage of `agency.agents.Agent`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryAgentStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::agents::AgentSnapshot>>>);
impl crate::behaviour::AgentStorage for MemoryAgentStorage {
fn get(&self, identity: &crate::agents::AgentId) -> Option<crate::agents::AgentSnapshot> { let key = memory_key_2(identity); self.0.borrow().iter().find(|row| memory_key_2(&row.data.agent_id) == key).cloned() }
fn put(&mut self, snapshot: crate::agents::AgentSnapshot) { self.delete(&snapshot.data.agent_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_2(&row.data.agent_id).cmp(&memory_key_2(&other.data.agent_id))); }
fn delete(&mut self, identity: &crate::agents::AgentId) { let key = memory_key_2(identity); self.0.borrow_mut().retain(|row| memory_key_2(&row.data.agent_id) != key); }
fn list(&self) -> Vec<crate::agents::AgentSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::AgentStorage for MemoryPorts {
fn get(&self, identity: &crate::agents::AgentId) -> Option<crate::agents::AgentSnapshot> { crate::behaviour::AgentStorage::get(&self.agent_storage, identity) }
fn put(&mut self, snapshot: crate::agents::AgentSnapshot) { crate::behaviour::AgentStorage::put(&mut self.agent_storage, snapshot); }
fn delete(&mut self, identity: &crate::agents::AgentId) { crate::behaviour::AgentStorage::delete(&mut self.agent_storage, identity); }
fn list(&self) -> Vec<crate::agents::AgentSnapshot> { crate::behaviour::AgentStorage::list(&self.agent_storage) }
}

/// Ephemeral storage of `agency.agents.AgentDefinition`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryAgentDefinitionStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::agents::AgentDefinitionSnapshot>>>);
impl crate::behaviour::AgentDefinitionStorage for MemoryAgentDefinitionStorage {
fn get(&self, identity: &crate::agents::DefinitionId) -> Option<crate::agents::AgentDefinitionSnapshot> { let key = memory_key_5(identity); self.0.borrow().iter().find(|row| memory_key_5(&row.data.definition_id) == key).cloned() }
fn put(&mut self, snapshot: crate::agents::AgentDefinitionSnapshot) { self.delete(&snapshot.data.definition_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_5(&row.data.definition_id).cmp(&memory_key_5(&other.data.definition_id))); }
fn delete(&mut self, identity: &crate::agents::DefinitionId) { let key = memory_key_5(identity); self.0.borrow_mut().retain(|row| memory_key_5(&row.data.definition_id) != key); }
fn list(&self) -> Vec<crate::agents::AgentDefinitionSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::AgentDefinitionStorage for MemoryPorts {
fn get(&self, identity: &crate::agents::DefinitionId) -> Option<crate::agents::AgentDefinitionSnapshot> { crate::behaviour::AgentDefinitionStorage::get(&self.agent_definition_storage, identity) }
fn put(&mut self, snapshot: crate::agents::AgentDefinitionSnapshot) { crate::behaviour::AgentDefinitionStorage::put(&mut self.agent_definition_storage, snapshot); }
fn delete(&mut self, identity: &crate::agents::DefinitionId) { crate::behaviour::AgentDefinitionStorage::delete(&mut self.agent_definition_storage, identity); }
fn list(&self) -> Vec<crate::agents::AgentDefinitionSnapshot> { crate::behaviour::AgentDefinitionStorage::list(&self.agent_definition_storage) }
}

/// Ephemeral storage of `agency.auth.Credential`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryCredentialStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::auth::CredentialSnapshot>>>);
impl crate::behaviour::CredentialStorage for MemoryCredentialStorage {
fn get(&self, identity: &crate::auth::CredentialId) -> Option<crate::auth::CredentialSnapshot> { let key = memory_key_9(identity); self.0.borrow().iter().find(|row| memory_key_9(&row.data.credential_id) == key).cloned() }
fn put(&mut self, snapshot: crate::auth::CredentialSnapshot) { self.delete(&snapshot.data.credential_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_9(&row.data.credential_id).cmp(&memory_key_9(&other.data.credential_id))); }
fn delete(&mut self, identity: &crate::auth::CredentialId) { let key = memory_key_9(identity); self.0.borrow_mut().retain(|row| memory_key_9(&row.data.credential_id) != key); }
fn list(&self) -> Vec<crate::auth::CredentialSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::CredentialStorage for MemoryPorts {
fn get(&self, identity: &crate::auth::CredentialId) -> Option<crate::auth::CredentialSnapshot> { crate::behaviour::CredentialStorage::get(&self.credential_storage, identity) }
fn put(&mut self, snapshot: crate::auth::CredentialSnapshot) { crate::behaviour::CredentialStorage::put(&mut self.credential_storage, snapshot); }
fn delete(&mut self, identity: &crate::auth::CredentialId) { crate::behaviour::CredentialStorage::delete(&mut self.credential_storage, identity); }
fn list(&self) -> Vec<crate::auth::CredentialSnapshot> { crate::behaviour::CredentialStorage::list(&self.credential_storage) }
}

/// Ephemeral storage of `agency.auth.CredentialGrant`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryCredentialGrantStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::auth::CredentialGrantSnapshot>>>);
impl crate::behaviour::CredentialGrantStorage for MemoryCredentialGrantStorage {
fn get(&self, identity: &crate::auth::GrantId) -> Option<crate::auth::CredentialGrantSnapshot> { let key = memory_key_15(identity); self.0.borrow().iter().find(|row| memory_key_15(&row.data.grant_id) == key).cloned() }
fn put(&mut self, snapshot: crate::auth::CredentialGrantSnapshot) { self.delete(&snapshot.data.grant_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_15(&row.data.grant_id).cmp(&memory_key_15(&other.data.grant_id))); }
fn delete(&mut self, identity: &crate::auth::GrantId) { let key = memory_key_15(identity); self.0.borrow_mut().retain(|row| memory_key_15(&row.data.grant_id) != key); }
fn list(&self) -> Vec<crate::auth::CredentialGrantSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::CredentialGrantStorage for MemoryPorts {
fn get(&self, identity: &crate::auth::GrantId) -> Option<crate::auth::CredentialGrantSnapshot> { crate::behaviour::CredentialGrantStorage::get(&self.credential_grant_storage, identity) }
fn put(&mut self, snapshot: crate::auth::CredentialGrantSnapshot) { crate::behaviour::CredentialGrantStorage::put(&mut self.credential_grant_storage, snapshot); }
fn delete(&mut self, identity: &crate::auth::GrantId) { crate::behaviour::CredentialGrantStorage::delete(&mut self.credential_grant_storage, identity); }
fn list(&self) -> Vec<crate::auth::CredentialGrantSnapshot> { crate::behaviour::CredentialGrantStorage::list(&self.credential_grant_storage) }
}

/// Ephemeral storage of `agency.auth.Policy`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryPolicyStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::auth::PolicySnapshot>>>);
impl crate::behaviour::PolicyStorage for MemoryPolicyStorage {
fn get(&self, identity: &crate::auth::PolicyId) -> Option<crate::auth::PolicySnapshot> { let key = memory_key_19(identity); self.0.borrow().iter().find(|row| memory_key_19(&row.data.policy_id) == key).cloned() }
fn put(&mut self, snapshot: crate::auth::PolicySnapshot) { self.delete(&snapshot.data.policy_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_19(&row.data.policy_id).cmp(&memory_key_19(&other.data.policy_id))); }
fn delete(&mut self, identity: &crate::auth::PolicyId) { let key = memory_key_19(identity); self.0.borrow_mut().retain(|row| memory_key_19(&row.data.policy_id) != key); }
fn list(&self) -> Vec<crate::auth::PolicySnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::PolicyStorage for MemoryPorts {
fn get(&self, identity: &crate::auth::PolicyId) -> Option<crate::auth::PolicySnapshot> { crate::behaviour::PolicyStorage::get(&self.policy_storage, identity) }
fn put(&mut self, snapshot: crate::auth::PolicySnapshot) { crate::behaviour::PolicyStorage::put(&mut self.policy_storage, snapshot); }
fn delete(&mut self, identity: &crate::auth::PolicyId) { crate::behaviour::PolicyStorage::delete(&mut self.policy_storage, identity); }
fn list(&self) -> Vec<crate::auth::PolicySnapshot> { crate::behaviour::PolicyStorage::list(&self.policy_storage) }
}

/// Ephemeral storage of `agency.auth.Principal`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryPrincipalStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::auth::PrincipalSnapshot>>>);
impl crate::behaviour::PrincipalStorage for MemoryPrincipalStorage {
fn get(&self, identity: &crate::auth::PrincipalId) -> Option<crate::auth::PrincipalSnapshot> { let key = memory_key_21(identity); self.0.borrow().iter().find(|row| memory_key_21(&row.data.principal_id) == key).cloned() }
fn put(&mut self, snapshot: crate::auth::PrincipalSnapshot) { self.delete(&snapshot.data.principal_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_21(&row.data.principal_id).cmp(&memory_key_21(&other.data.principal_id))); }
fn delete(&mut self, identity: &crate::auth::PrincipalId) { let key = memory_key_21(identity); self.0.borrow_mut().retain(|row| memory_key_21(&row.data.principal_id) != key); }
fn list(&self) -> Vec<crate::auth::PrincipalSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::PrincipalStorage for MemoryPorts {
fn get(&self, identity: &crate::auth::PrincipalId) -> Option<crate::auth::PrincipalSnapshot> { crate::behaviour::PrincipalStorage::get(&self.principal_storage, identity) }
fn put(&mut self, snapshot: crate::auth::PrincipalSnapshot) { crate::behaviour::PrincipalStorage::put(&mut self.principal_storage, snapshot); }
fn delete(&mut self, identity: &crate::auth::PrincipalId) { crate::behaviour::PrincipalStorage::delete(&mut self.principal_storage, identity); }
fn list(&self) -> Vec<crate::auth::PrincipalSnapshot> { crate::behaviour::PrincipalStorage::list(&self.principal_storage) }
}

/// Ephemeral storage of `agency.auth.Tenant`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryTenantStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::auth::TenantSnapshot>>>);
impl crate::behaviour::TenantStorage for MemoryTenantStorage {
fn get(&self, identity: &crate::auth::TenantId) -> Option<crate::auth::TenantSnapshot> { let key = memory_key_29(identity); self.0.borrow().iter().find(|row| memory_key_29(&row.data.tenant_id) == key).cloned() }
fn put(&mut self, snapshot: crate::auth::TenantSnapshot) { self.delete(&snapshot.data.tenant_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_29(&row.data.tenant_id).cmp(&memory_key_29(&other.data.tenant_id))); }
fn delete(&mut self, identity: &crate::auth::TenantId) { let key = memory_key_29(identity); self.0.borrow_mut().retain(|row| memory_key_29(&row.data.tenant_id) != key); }
}
impl crate::behaviour::TenantStorage for MemoryPorts {
fn get(&self, identity: &crate::auth::TenantId) -> Option<crate::auth::TenantSnapshot> { crate::behaviour::TenantStorage::get(&self.tenant_storage, identity) }
fn put(&mut self, snapshot: crate::auth::TenantSnapshot) { crate::behaviour::TenantStorage::put(&mut self.tenant_storage, snapshot); }
fn delete(&mut self, identity: &crate::auth::TenantId) { crate::behaviour::TenantStorage::delete(&mut self.tenant_storage, identity); }
}

/// Ephemeral storage of `agency.runs.Run`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryRunStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::runs::RunSnapshot>>>);
impl crate::behaviour::RunStorage for MemoryRunStorage {
fn get(&self, identity: &crate::runs::RunId) -> Option<crate::runs::RunSnapshot> { let key = memory_key_39(identity); self.0.borrow().iter().find(|row| memory_key_39(&row.data.run_id) == key).cloned() }
fn put(&mut self, snapshot: crate::runs::RunSnapshot) { self.delete(&snapshot.data.run_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_39(&row.data.run_id).cmp(&memory_key_39(&other.data.run_id))); }
fn delete(&mut self, identity: &crate::runs::RunId) { let key = memory_key_39(identity); self.0.borrow_mut().retain(|row| memory_key_39(&row.data.run_id) != key); }
fn list(&self) -> Vec<crate::runs::RunSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::RunStorage for MemoryPorts {
fn get(&self, identity: &crate::runs::RunId) -> Option<crate::runs::RunSnapshot> { crate::behaviour::RunStorage::get(&self.run_storage, identity) }
fn put(&mut self, snapshot: crate::runs::RunSnapshot) { crate::behaviour::RunStorage::put(&mut self.run_storage, snapshot); }
fn delete(&mut self, identity: &crate::runs::RunId) { crate::behaviour::RunStorage::delete(&mut self.run_storage, identity); }
fn list(&self) -> Vec<crate::runs::RunSnapshot> { crate::behaviour::RunStorage::list(&self.run_storage) }
}

/// Ephemeral storage of `agency.runs.RunApproval`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryRunApprovalStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::runs::RunApprovalSnapshot>>>);
impl crate::behaviour::RunApprovalStorage for MemoryRunApprovalStorage {
fn get(&self, identity: &crate::runs::ApprovalId) -> Option<crate::runs::RunApprovalSnapshot> { let key = memory_key_33(identity); self.0.borrow().iter().find(|row| memory_key_33(&row.data.approval_id) == key).cloned() }
fn put(&mut self, snapshot: crate::runs::RunApprovalSnapshot) { self.delete(&snapshot.data.approval_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_33(&row.data.approval_id).cmp(&memory_key_33(&other.data.approval_id))); }
fn delete(&mut self, identity: &crate::runs::ApprovalId) { let key = memory_key_33(identity); self.0.borrow_mut().retain(|row| memory_key_33(&row.data.approval_id) != key); }
fn list(&self) -> Vec<crate::runs::RunApprovalSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::RunApprovalStorage for MemoryPorts {
fn get(&self, identity: &crate::runs::ApprovalId) -> Option<crate::runs::RunApprovalSnapshot> { crate::behaviour::RunApprovalStorage::get(&self.run_approval_storage, identity) }
fn put(&mut self, snapshot: crate::runs::RunApprovalSnapshot) { crate::behaviour::RunApprovalStorage::put(&mut self.run_approval_storage, snapshot); }
fn delete(&mut self, identity: &crate::runs::ApprovalId) { crate::behaviour::RunApprovalStorage::delete(&mut self.run_approval_storage, identity); }
fn list(&self) -> Vec<crate::runs::RunApprovalSnapshot> { crate::behaviour::RunApprovalStorage::list(&self.run_approval_storage) }
}

/// Ephemeral storage of `agency.service.Session`, shared by clones.
#[derive(Clone, Default)]
pub struct MemorySessionStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::service::SessionSnapshot>>>);
impl crate::behaviour::SessionStorage for MemorySessionStorage {
fn get(&self, identity: &crate::service::SessionId) -> Option<crate::service::SessionSnapshot> { let key = memory_key_44(identity); self.0.borrow().iter().find(|row| memory_key_44(&row.data.session_id) == key).cloned() }
fn put(&mut self, snapshot: crate::service::SessionSnapshot) { self.delete(&snapshot.data.session_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_44(&row.data.session_id).cmp(&memory_key_44(&other.data.session_id))); }
fn delete(&mut self, identity: &crate::service::SessionId) { let key = memory_key_44(identity); self.0.borrow_mut().retain(|row| memory_key_44(&row.data.session_id) != key); }
fn list(&self) -> Vec<crate::service::SessionSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::SessionStorage for MemoryPorts {
fn get(&self, identity: &crate::service::SessionId) -> Option<crate::service::SessionSnapshot> { crate::behaviour::SessionStorage::get(&self.session_storage, identity) }
fn put(&mut self, snapshot: crate::service::SessionSnapshot) { crate::behaviour::SessionStorage::put(&mut self.session_storage, snapshot); }
fn delete(&mut self, identity: &crate::service::SessionId) { crate::behaviour::SessionStorage::delete(&mut self.session_storage, identity); }
fn list(&self) -> Vec<crate::service::SessionSnapshot> { crate::behaviour::SessionStorage::list(&self.session_storage) }
}

/// Ephemeral storage of `agency.teams.Swarm`, shared by clones.
#[derive(Clone, Default)]
pub struct MemorySwarmStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::teams::SwarmSnapshot>>>);
impl crate::behaviour::SwarmStorage for MemorySwarmStorage {
fn get(&self, identity: &crate::teams::SwarmId) -> Option<crate::teams::SwarmSnapshot> { let key = memory_key_51(identity); self.0.borrow().iter().find(|row| memory_key_51(&row.data.swarm_id) == key).cloned() }
fn put(&mut self, snapshot: crate::teams::SwarmSnapshot) { self.delete(&snapshot.data.swarm_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_51(&row.data.swarm_id).cmp(&memory_key_51(&other.data.swarm_id))); }
fn delete(&mut self, identity: &crate::teams::SwarmId) { let key = memory_key_51(identity); self.0.borrow_mut().retain(|row| memory_key_51(&row.data.swarm_id) != key); }
fn list(&self) -> Vec<crate::teams::SwarmSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::SwarmStorage for MemoryPorts {
fn get(&self, identity: &crate::teams::SwarmId) -> Option<crate::teams::SwarmSnapshot> { crate::behaviour::SwarmStorage::get(&self.swarm_storage, identity) }
fn put(&mut self, snapshot: crate::teams::SwarmSnapshot) { crate::behaviour::SwarmStorage::put(&mut self.swarm_storage, snapshot); }
fn delete(&mut self, identity: &crate::teams::SwarmId) { crate::behaviour::SwarmStorage::delete(&mut self.swarm_storage, identity); }
fn list(&self) -> Vec<crate::teams::SwarmSnapshot> { crate::behaviour::SwarmStorage::list(&self.swarm_storage) }
}

/// Ephemeral storage of `agency.teams.Team`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryTeamStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::teams::TeamSnapshot>>>);
impl crate::behaviour::TeamStorage for MemoryTeamStorage {
fn get(&self, identity: &crate::teams::TeamId) -> Option<crate::teams::TeamSnapshot> { let key = memory_key_53(identity); self.0.borrow().iter().find(|row| memory_key_53(&row.data.team_id) == key).cloned() }
fn put(&mut self, snapshot: crate::teams::TeamSnapshot) { self.delete(&snapshot.data.team_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_53(&row.data.team_id).cmp(&memory_key_53(&other.data.team_id))); }
fn delete(&mut self, identity: &crate::teams::TeamId) { let key = memory_key_53(identity); self.0.borrow_mut().retain(|row| memory_key_53(&row.data.team_id) != key); }
fn list(&self) -> Vec<crate::teams::TeamSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::TeamStorage for MemoryPorts {
fn get(&self, identity: &crate::teams::TeamId) -> Option<crate::teams::TeamSnapshot> { crate::behaviour::TeamStorage::get(&self.team_storage, identity) }
fn put(&mut self, snapshot: crate::teams::TeamSnapshot) { crate::behaviour::TeamStorage::put(&mut self.team_storage, snapshot); }
fn delete(&mut self, identity: &crate::teams::TeamId) { crate::behaviour::TeamStorage::delete(&mut self.team_storage, identity); }
fn list(&self) -> Vec<crate::teams::TeamSnapshot> { crate::behaviour::TeamStorage::list(&self.team_storage) }
}

/// Ephemeral storage of `agency.teams.Workforce`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryWorkforceStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::teams::WorkforceSnapshot>>>);
impl crate::behaviour::WorkforceStorage for MemoryWorkforceStorage {
fn get(&self, identity: &crate::teams::WorkforceId) -> Option<crate::teams::WorkforceSnapshot> { let key = memory_key_55(identity); self.0.borrow().iter().find(|row| memory_key_55(&row.data.workforce_id) == key).cloned() }
fn put(&mut self, snapshot: crate::teams::WorkforceSnapshot) { self.delete(&snapshot.data.workforce_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_55(&row.data.workforce_id).cmp(&memory_key_55(&other.data.workforce_id))); }
fn delete(&mut self, identity: &crate::teams::WorkforceId) { let key = memory_key_55(identity); self.0.borrow_mut().retain(|row| memory_key_55(&row.data.workforce_id) != key); }
fn list(&self) -> Vec<crate::teams::WorkforceSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::WorkforceStorage for MemoryPorts {
fn get(&self, identity: &crate::teams::WorkforceId) -> Option<crate::teams::WorkforceSnapshot> { crate::behaviour::WorkforceStorage::get(&self.workforce_storage, identity) }
fn put(&mut self, snapshot: crate::teams::WorkforceSnapshot) { crate::behaviour::WorkforceStorage::put(&mut self.workforce_storage, snapshot); }
fn delete(&mut self, identity: &crate::teams::WorkforceId) { crate::behaviour::WorkforceStorage::delete(&mut self.workforce_storage, identity); }
fn list(&self) -> Vec<crate::teams::WorkforceSnapshot> { crate::behaviour::WorkforceStorage::list(&self.workforce_storage) }
}

/// Ephemeral storage of `agency.tools.Tool`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryToolStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::tools::ToolSnapshot>>>);
impl crate::behaviour::ToolStorage for MemoryToolStorage {
fn get(&self, identity: &crate::tools::ToolId) -> Option<crate::tools::ToolSnapshot> { let key = memory_key_62(identity); self.0.borrow().iter().find(|row| memory_key_62(&row.data.tool_id) == key).cloned() }
fn put(&mut self, snapshot: crate::tools::ToolSnapshot) { self.delete(&snapshot.data.tool_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_62(&row.data.tool_id).cmp(&memory_key_62(&other.data.tool_id))); }
fn delete(&mut self, identity: &crate::tools::ToolId) { let key = memory_key_62(identity); self.0.borrow_mut().retain(|row| memory_key_62(&row.data.tool_id) != key); }
fn list(&self) -> Vec<crate::tools::ToolSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::ToolStorage for MemoryPorts {
fn get(&self, identity: &crate::tools::ToolId) -> Option<crate::tools::ToolSnapshot> { crate::behaviour::ToolStorage::get(&self.tool_storage, identity) }
fn put(&mut self, snapshot: crate::tools::ToolSnapshot) { crate::behaviour::ToolStorage::put(&mut self.tool_storage, snapshot); }
fn delete(&mut self, identity: &crate::tools::ToolId) { crate::behaviour::ToolStorage::delete(&mut self.tool_storage, identity); }
fn list(&self) -> Vec<crate::tools::ToolSnapshot> { crate::behaviour::ToolStorage::list(&self.tool_storage) }
}

/// Ephemeral storage of `agency.tools.ToolBinding`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryToolBindingStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::tools::ToolBindingSnapshot>>>);
impl crate::behaviour::ToolBindingStorage for MemoryToolBindingStorage {
fn get(&self, identity: &crate::tools::BindingId) -> Option<crate::tools::ToolBindingSnapshot> { let key = memory_key_56(identity); self.0.borrow().iter().find(|row| memory_key_56(&row.data.binding_id) == key).cloned() }
fn put(&mut self, snapshot: crate::tools::ToolBindingSnapshot) { self.delete(&snapshot.data.binding_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_56(&row.data.binding_id).cmp(&memory_key_56(&other.data.binding_id))); }
fn delete(&mut self, identity: &crate::tools::BindingId) { let key = memory_key_56(identity); self.0.borrow_mut().retain(|row| memory_key_56(&row.data.binding_id) != key); }
fn list(&self) -> Vec<crate::tools::ToolBindingSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::ToolBindingStorage for MemoryPorts {
fn get(&self, identity: &crate::tools::BindingId) -> Option<crate::tools::ToolBindingSnapshot> { crate::behaviour::ToolBindingStorage::get(&self.tool_binding_storage, identity) }
fn put(&mut self, snapshot: crate::tools::ToolBindingSnapshot) { crate::behaviour::ToolBindingStorage::put(&mut self.tool_binding_storage, snapshot); }
fn delete(&mut self, identity: &crate::tools::BindingId) { crate::behaviour::ToolBindingStorage::delete(&mut self.tool_binding_storage, identity); }
fn list(&self) -> Vec<crate::tools::ToolBindingSnapshot> { crate::behaviour::ToolBindingStorage::list(&self.tool_binding_storage) }
}

/// Ephemeral storage of `agency.work.WorkInput`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryWorkInputStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::work::WorkInputSnapshot>>>);
impl crate::behaviour::WorkInputStorage for MemoryWorkInputStorage {
fn get(&self, identity: &crate::work::InputId) -> Option<crate::work::WorkInputSnapshot> { let key = memory_key_66(identity); self.0.borrow().iter().find(|row| memory_key_66(&row.data.input_id) == key).cloned() }
fn put(&mut self, snapshot: crate::work::WorkInputSnapshot) { self.delete(&snapshot.data.input_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_66(&row.data.input_id).cmp(&memory_key_66(&other.data.input_id))); }
fn delete(&mut self, identity: &crate::work::InputId) { let key = memory_key_66(identity); self.0.borrow_mut().retain(|row| memory_key_66(&row.data.input_id) != key); }
fn list(&self) -> Vec<crate::work::WorkInputSnapshot> { self.0.borrow().clone() }
}
impl crate::behaviour::WorkInputStorage for MemoryPorts {
fn get(&self, identity: &crate::work::InputId) -> Option<crate::work::WorkInputSnapshot> { crate::behaviour::WorkInputStorage::get(&self.work_input_storage, identity) }
fn put(&mut self, snapshot: crate::work::WorkInputSnapshot) { crate::behaviour::WorkInputStorage::put(&mut self.work_input_storage, snapshot); }
fn delete(&mut self, identity: &crate::work::InputId) { crate::behaviour::WorkInputStorage::delete(&mut self.work_input_storage, identity); }
fn list(&self) -> Vec<crate::work::WorkInputSnapshot> { crate::behaviour::WorkInputStorage::list(&self.work_input_storage) }
}

/// Ephemeral storage of `agency.work.WorkItem`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryWorkItemStorage(std::rc::Rc<std::cell::RefCell<Vec<crate::work::WorkItemSnapshot>>>);
impl crate::behaviour::WorkItemStorage for MemoryWorkItemStorage {
fn get(&self, identity: &crate::work::WorkItemRef) -> Option<crate::work::WorkItemSnapshot> { let key = memory_key_70(identity); self.0.borrow().iter().find(|row| memory_key_70(&row.data.item_ref) == key).cloned() }
fn put(&mut self, snapshot: crate::work::WorkItemSnapshot) { self.delete(&snapshot.data.item_ref); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_70(&row.data.item_ref).cmp(&memory_key_70(&other.data.item_ref))); }
fn delete(&mut self, identity: &crate::work::WorkItemRef) { let key = memory_key_70(identity); self.0.borrow_mut().retain(|row| memory_key_70(&row.data.item_ref) != key); }
}
impl crate::behaviour::WorkItemStorage for MemoryPorts {
fn get(&self, identity: &crate::work::WorkItemRef) -> Option<crate::work::WorkItemSnapshot> { crate::behaviour::WorkItemStorage::get(&self.work_item_storage, identity) }
fn put(&mut self, snapshot: crate::work::WorkItemSnapshot) { crate::behaviour::WorkItemStorage::put(&mut self.work_item_storage, snapshot); }
fn delete(&mut self, identity: &crate::work::WorkItemRef) { crate::behaviour::WorkItemStorage::delete(&mut self.work_item_storage, identity); }
}

/// Ports shared by generated components. Values disappear when the process exits.
#[derive(Clone, Default)]
pub struct MemoryPorts {
/// Storage of `agency.agents.Agent`.
pub agent_storage: MemoryAgentStorage,
/// Storage of `agency.agents.AgentDefinition`.
pub agent_definition_storage: MemoryAgentDefinitionStorage,
/// Storage of `agency.auth.Credential`.
pub credential_storage: MemoryCredentialStorage,
/// Storage of `agency.auth.CredentialGrant`.
pub credential_grant_storage: MemoryCredentialGrantStorage,
/// Storage of `agency.auth.Policy`.
pub policy_storage: MemoryPolicyStorage,
/// Storage of `agency.auth.Principal`.
pub principal_storage: MemoryPrincipalStorage,
/// Storage of `agency.auth.Tenant`.
pub tenant_storage: MemoryTenantStorage,
/// Storage of `agency.runs.Run`.
pub run_storage: MemoryRunStorage,
/// Storage of `agency.runs.RunApproval`.
pub run_approval_storage: MemoryRunApprovalStorage,
/// Storage of `agency.service.Session`.
pub session_storage: MemorySessionStorage,
/// Storage of `agency.teams.Swarm`.
pub swarm_storage: MemorySwarmStorage,
/// Storage of `agency.teams.Team`.
pub team_storage: MemoryTeamStorage,
/// Storage of `agency.teams.Workforce`.
pub workforce_storage: MemoryWorkforceStorage,
/// Storage of `agency.tools.Tool`.
pub tool_storage: MemoryToolStorage,
/// Storage of `agency.tools.ToolBinding`.
pub tool_binding_storage: MemoryToolBindingStorage,
/// Storage of `agency.work.WorkInput`.
pub work_input_storage: MemoryWorkInputStorage,
/// Storage of `agency.work.WorkItem`.
pub work_item_storage: MemoryWorkItemStorage,
}

impl crate::behaviour::TryContext for MemoryPorts {
fn try_generate_agency_agents_agent_id(&mut self) -> Result<crate::agents::AgentId, crate::obligation::UnmetObligation> { Ok(crate::agents::AgentId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_agents_definition_id(&mut self) -> Result<crate::agents::DefinitionId, crate::obligation::UnmetObligation> { Ok(crate::agents::DefinitionId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_auth_credential_id(&mut self) -> Result<crate::auth::CredentialId, crate::obligation::UnmetObligation> { Ok(crate::auth::CredentialId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_auth_grant_id(&mut self) -> Result<crate::auth::GrantId, crate::obligation::UnmetObligation> { Ok(crate::auth::GrantId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_auth_policy_id(&mut self) -> Result<crate::auth::PolicyId, crate::obligation::UnmetObligation> { Ok(crate::auth::PolicyId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_auth_principal_id(&mut self) -> Result<crate::auth::PrincipalId, crate::obligation::UnmetObligation> { Ok(crate::auth::PrincipalId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_auth_tenant_id(&mut self) -> Result<crate::auth::TenantId, crate::obligation::UnmetObligation> { Ok(crate::auth::TenantId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_runs_approval_id(&mut self) -> Result<crate::runs::ApprovalId, crate::obligation::UnmetObligation> { Ok(crate::runs::ApprovalId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_runs_run_id(&mut self) -> Result<crate::runs::RunId, crate::obligation::UnmetObligation> { Ok(crate::runs::RunId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_service_session_id(&mut self) -> Result<crate::service::SessionId, crate::obligation::UnmetObligation> { Ok(crate::service::SessionId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_shared_digest(&mut self) -> Result<crate::shared::Digest, crate::obligation::UnmetObligation> { Err(crate::behaviour::unmet_context("assigned value: agency.shared.Digest")) }
fn try_generate_agency_teams_swarm_id(&mut self) -> Result<crate::teams::SwarmId, crate::obligation::UnmetObligation> { Ok(crate::teams::SwarmId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_teams_team_id(&mut self) -> Result<crate::teams::TeamId, crate::obligation::UnmetObligation> { Ok(crate::teams::TeamId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_teams_workforce_id(&mut self) -> Result<crate::teams::WorkforceId, crate::obligation::UnmetObligation> { Ok(crate::teams::WorkforceId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_tools_binding_id(&mut self) -> Result<crate::tools::BindingId, crate::obligation::UnmetObligation> { Ok(crate::tools::BindingId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_tools_tool_id(&mut self) -> Result<crate::tools::ToolId, crate::obligation::UnmetObligation> { Ok(crate::tools::ToolId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_agency_work_input_id(&mut self) -> Result<crate::work::InputId, crate::obligation::UnmetObligation> { Ok(crate::work::InputId(crate::primitives::Uuid(uuid::Uuid::new_v4().to_string()))) }
fn try_generate_integer(&mut self) -> Result<i64, crate::obligation::UnmetObligation> { Err(crate::behaviour::unmet_context("assigned value: Integer")) }
fn try_generate_optional_timestamp(&mut self) -> Result<Option<crate::primitives::Timestamp>, crate::obligation::UnmetObligation> { Ok(None) }
fn try_generate_timestamp(&mut self) -> Result<crate::primitives::Timestamp, crate::obligation::UnmetObligation> { Ok(crate::primitives::Timestamp(time::OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).expect("UTC time formats"))) }
fn try_external(&mut self, _command: crate::behaviour::ExternalCommand<'_>, _outcome: &'static str) -> Result<bool, crate::obligation::UnmetObligation> { Err(crate::behaviour::unmet_context("external branch answer")) }
}

fn memory_key_2(value: &crate::agents::AgentId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_5(value: &crate::agents::DefinitionId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_9(value: &crate::auth::CredentialId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_15(value: &crate::auth::GrantId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_19(value: &crate::auth::PolicyId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_21(value: &crate::auth::PrincipalId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_29(value: &crate::auth::TenantId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_33(value: &crate::runs::ApprovalId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_39(value: &crate::runs::RunId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_44(value: &crate::service::SessionId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_51(value: &crate::teams::SwarmId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_53(value: &crate::teams::TeamId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_55(value: &crate::teams::WorkforceId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_56(value: &crate::tools::BindingId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_62(value: &crate::tools::ToolId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_66(value: &crate::work::InputId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).0.clone()) }

fn memory_key_70(value: &crate::work::WorkItemRef) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

// A common structural order; individual specifications use only a subset of the variants.
#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum MemoryKey {
    Null,
    Boolean(bool),
    Integer(i64),
    Number(String),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<MemoryKey>),
    Map(Vec<(MemoryKey, MemoryKey)>),
    Optional(Option<Box<MemoryKey>>),
}

impl crate::auth::obligations::EffectivePolicyQuery for MemoryPorts {
fn effective_policy(&self, definition_id: crate::agents::DefinitionId) -> Result<Vec<crate::auth::EffectivePolicy>, crate::obligation::UnmetObligation> { crate::auth::obligations::EffectivePolicyQuery::effective_policy(&crate::auth::obligations::Unimplemented, definition_id) }
}
impl crate::runs::obligations::RunDetailQuery for MemoryPorts {
fn run_detail(&self, run_id: crate::runs::RunId) -> Result<Vec<crate::runs::RunDetail>, crate::obligation::UnmetObligation> { crate::runs::obligations::RunDetailQuery::run_detail(&crate::runs::obligations::Unimplemented, run_id) }
}
impl crate::teams::obligations::TeamDetailQuery for MemoryPorts {
fn team_detail(&self, team_id: crate::teams::TeamId) -> Result<Vec<crate::teams::TeamDetail>, crate::obligation::UnmetObligation> { crate::teams::obligations::TeamDetailQuery::team_detail(&crate::teams::obligations::Unimplemented, team_id) }
}
impl crate::work::obligations::WorkItemDetailQuery for MemoryPorts {
fn work_item_detail(&self, item_ref: crate::work::WorkItemRef) -> Result<Vec<crate::work::WorkItemDetail>, crate::obligation::UnmetObligation> { crate::work::obligations::WorkItemDetailQuery::work_item_detail(&crate::work::obligations::Unimplemented, item_ref) }
}
impl crate::work::obligations::WorkQueueQuery for MemoryPorts {
fn work_queue(&self) -> Result<Vec<crate::work::WorkQueue>, crate::obligation::UnmetObligation> { crate::work::obligations::WorkQueueQuery::work_queue(&crate::work::obligations::Unimplemented) }
}
