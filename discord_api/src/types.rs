use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use nectar_process_lib::print_to_terminal;

pub const DISCORD_GATEWAY: &str = "wss://gateway.discord.gg/?v=9&encoding=json";
pub const HTTP_URL: &str = "https://discord.com/api/v9";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectGateway {
    pub bot_token: String,
    pub intents: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayPayload {
    pub op: u32,
    pub d: serde_json::Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayIdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityTimestamps {
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityEmoji {
    name: String,
    id: Option<u64>,
    animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityParty {
    id: Option<String>,
    size: Option<[u64; 2]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivitySecrets {
    join: Option<String>,
    spectate: Option<String>,
    match_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivity {
    name: String,
    #[serde(rename = "type")]
    activity_type: u32,
    url: Option<String>,
    created_at: u64,
    timestamps: Option<GatewayActivityTimestamps>,
    application_id: Option<u64>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<GatewayActivityEmoji>,
    party: Option<GatewayActivityParty>,
    assets: Option<GatewayActivityAssets>,
    secrets: Option<GatewayActivitySecrets>,
    instance: Option<bool>,
    flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayPresenceUpdate {
    since: Option<u64>,
    activities: Option<Vec<GatewayActivity>>,
    status: String,
    afk: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewaySendEvent {
    Identify {
        token: String,
        properties: GatewayIdentifyProperties,
        compress: Option<bool>,
        large_threshold: Option<u64>,
        shard: Option<[u64; 2]>,
        presence: Option<GatewayPresenceUpdate>,
        guild_subscriptions: Option<bool>,
        intents: u128,
    },
    Resume {
        token: String,
        session_id: String,
        seq: u64,
    },
    Heartbeat {
        seq: Option<u64>,
    },
    RequestGuildMembers {
        guild_id: String,
        query: Option<String>,
        limit: u64,
        presences: Option<bool>,
        user_ids: Option<Vec<String>>,
        nonce: String,
    },
    UpdateVoiceState {
        guild_id: String,
        channel_id: Option<String>,
        self_mute: bool,
        self_deaf: bool,
    },
    UpdatePresence {
        since: Option<u64>,
        activities: Option<Vec<GatewayActivity>>,
        status: String,
        afk: bool,
    }
}

impl GatewaySendEvent {
    pub fn to_json_bytes(&self) -> Vec<u8> {
        match self {
            // Convert to JSON
            GatewaySendEvent::Identify { token, properties, compress, large_threshold, shard, presence, guild_subscriptions, intents } => {
                serde_json::json!({
                    "op": 2,
                    "d": {
                        "token": token,
                        "properties": properties,
                        "compress": compress.unwrap_or(false),
                        "large_threshold": large_threshold.unwrap_or(50),
                        "shard": shard.unwrap_or([0, 1]),
                        "presence": presence,
                        "guild_subscriptions": guild_subscriptions,
                        "intents": intents,
                    },
                }).to_string().as_bytes().to_vec()
            },
            GatewaySendEvent::Resume { token, session_id, seq } => {
                serde_json::json!({
                    "op": 6,
                    "d": {
                        "token": token,
                        "session_id": session_id,
                        "seq": seq,
                    },
                }).to_string().as_bytes().to_vec()
            },
            GatewaySendEvent::Heartbeat { seq } => {
                serde_json::json!({
                    "op": 1,
                    "d": seq,
                }).to_string().as_bytes().to_vec()
            },
            GatewaySendEvent::RequestGuildMembers { guild_id, query, limit, presences, user_ids, nonce } => {
                serde_json::json!({
                    "op": 8,
                    "d": {
                        "guild_id": guild_id,
                        "query": query,
                        "limit": limit,
                        "presences": presences,
                        "user_ids": user_ids,
                        "nonce": nonce,
                    },
                }).to_string().as_bytes().to_vec()
            },
            GatewaySendEvent::UpdateVoiceState { guild_id, channel_id, self_mute, self_deaf } => {
                serde_json::json!({
                    "op": 4,
                    "d": {
                        "guild_id": guild_id,
                        "channel_id": channel_id,
                        "self_mute": self_mute,
                        "self_deaf": self_deaf,
                    },
                }).to_string().as_bytes().to_vec()
            },
            GatewaySendEvent::UpdatePresence { since, activities, status, afk } => {
                serde_json::json!({
                    "op": 3,
                    "d": {
                        "since": since,
                        "activities": activities,
                        "status": status,
                        "afk": afk,
                    },
                }).to_string().as_bytes().to_vec()
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewayEventType {
  Ready,
  Resumed,
  ApplicationCommandPermissionsUpdate,
  AutoModerationRuleCreate,
  AutoModerationRuleUpdate,
  AutoModerationRuleDelete,
  AutoModerationActionExecution,
  ChannelCreate,
  ChannelUpdate,
  ChannelDelete,
  ChannelPinsUpdate,
  ThreadCreate,
  ThreadUpdate,
  ThreadDelete,
  ThreadListSync,
  ThreadMemberUpdate,
  ThreadMembersUpdate,
  EntitlementCreate,
  EntitlementUpdate,
  EntitlementDelete,
  GuildCreate,
  GuildUpdate,
  GuildDelete,
  GuildAuditLogEntryCreate,
  GuildBanAdd,
  GuildBanRemove,
  GuildEmojisUpdate,
  GuildStickersUpdate,
  GuildIntegrationsUpdate,
  GuildMemberAdd,
  GuildMemberRemove,
  GuildMemberUpdate,
  GuildMembersChunk,
  GuildRoleCreate,
  GuildRoleUpdate,
  GuildRoleDelete,
  GuildScheduledEventCreate,
  GuildScheduledEventUpdate,
  GuildScheduledEventDelete,
  GuildScheduledEventUserAdd,
  GuildScheduledEventUserRemove,
  IntegrationCreate,
  IntegrationUpdate,
  IntegrationDelete,
  InteractionCreate,
  InviteCreate,
  InviteDelete,
  MessageCreate,
  MessageUpdate,
  MessageDelete,
  MessageDeleteBulk,
  MessageReactionAdd,
  MessageReactionRemove,
  MessageReactionRemoveAll,
  MessageReactionRemoveEmoji,
  PresenceUpdate,
  StageInstanceCreate,
  StageInstanceUpdate,
  StageInstanceDelete,
  TypingStart,
  UserUpdate,
  VoiceStateUpdate,
  VoiceServerUpdate,
  WebhooksUpdate,
}

impl GatewayEventType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "READY" => Some(Self::Ready),
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => Some(Self::ApplicationCommandPermissionsUpdate),
            "AUTOMODERATION_RULE_CREATE" => Some(Self::AutoModerationRuleCreate),
            "AUTOMODERATION_RULE_UPDATE" => Some(Self::AutoModerationRuleUpdate),
            "AUTOMODERATION_RULE_DELETE" => Some(Self::AutoModerationRuleDelete),
            "AUTOMODERATION_ACTION_EXECUTION" => Some(Self::AutoModerationActionExecution),
            "CHANNEL_CREATE" => Some(Self::ChannelCreate),
            "CHANNEL_UPDATE" => Some(Self::ChannelUpdate),
            "CHANNEL_DELETE" => Some(Self::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => Some(Self::ChannelPinsUpdate),
            "THREAD_CREATE" => Some(Self::ThreadCreate),
            "THREAD_UPDATE" => Some(Self::ThreadUpdate),
            "THREAD_DELETE" => Some(Self::ThreadDelete),
            "THREAD_LIST_SYNC" => Some(Self::ThreadListSync),
            "THREAD_MEMBER_UPDATE" => Some(Self::ThreadMemberUpdate),
            "THREAD_MEMBERS_UPDATE" => Some(Self::ThreadMembersUpdate),
            "ENTITLEMENT_CREATE" => Some(Self::EntitlementCreate),
            "ENTITLEMENT_UPDATE" => Some(Self::EntitlementUpdate),
            "ENTITLEMENT_DELETE" => Some(Self::EntitlementDelete),
            "GUILD_CREATE" => Some(Self::GuildCreate),
            "GUILD_UPDATE" => Some(Self::GuildUpdate),
            "GUILD_DELETE" => Some(Self::GuildDelete),
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => Some(Self::GuildAuditLogEntryCreate),
            "GUILD_BAN_ADD" => Some(Self::GuildBanAdd),
            "GUILD_BAN_REMOVE" => Some(Self::GuildBanRemove),
            "GUILD_EMOJIS_UPDATE" => Some(Self::GuildEmojisUpdate),
            "GUILD_STICKERS_UPDATE" => Some(Self::GuildStickersUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => Some(Self::GuildIntegrationsUpdate),
            "GUILD_MEMBER_ADD" => Some(Self::GuildMemberAdd),
            "GUILD_MEMBER_REMOVE" => Some(Self::GuildMemberRemove),
            "GUILD_MEMBER_UPDATE" => Some(Self::GuildMemberUpdate),
            "GUILD_MEMBERS_CHUNK" => Some(Self::GuildMembersChunk),
            "GUILD_ROLE_CREATE" => Some(Self::GuildRoleCreate),
            "GUILD_ROLE_UPDATE" => Some(Self::GuildRoleUpdate),
            "GUILD_ROLE_DELETE" => Some(Self::GuildRoleDelete),
            "GUILD_SCHEDULED_EVENT_CREATE" => Some(Self::GuildScheduledEventCreate),
            "GUILD_SCHEDULED_EVENT_UPDATE" => Some(Self::GuildScheduledEventUpdate),
            "GUILD_SCHEDULED_EVENT_DELETE" => Some(Self::GuildScheduledEventDelete),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => Some(Self::GuildScheduledEventUserAdd),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => Some(Self::GuildScheduledEventUserRemove),
            "INTEGRATION_CREATE" => Some(Self::IntegrationCreate),
            "INTEGRATION_UPDATE" => Some(Self::IntegrationUpdate),
            "INTEGRATION_DELETE" => Some(Self::IntegrationDelete),
            "INTERACTION_CREATE" => Some(Self::InteractionCreate),
            "INVITE_CREATE" => Some(Self::InviteCreate),
            "INVITE_DELETE" => Some(Self::InviteDelete),
            "MESSAGE_CREATE" => Some(Self::MessageCreate),
            "MESSAGE_UPDATE" => Some(Self::MessageUpdate),
            "MESSAGE_DELETE" => Some(Self::MessageDelete),
            "MESSAGE_DELETE_BULK" => Some(Self::MessageDeleteBulk),
            "MESSAGE_REACTION_ADD" => Some(Self::MessageReactionAdd),
            "MESSAGE_REACTION_REMOVE" => Some(Self::MessageReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => Some(Self::MessageReactionRemoveAll),
            "MESSAGE_REACTION_REMOVE_EMOJI" => Some(Self::MessageReactionRemoveEmoji),
            "PRESENCE_UPDATE" => Some(Self::PresenceUpdate),
            "STAGE_INSTANCE_CREATE" => Some(Self::StageInstanceCreate),
            "STAGE_INSTANCE_UPDATE" => Some(Self::StageInstanceUpdate),
            "STAGE_INSTANCE_DELETE" => Some(Self::StageInstanceDelete),
            "TYPING_START" => Some(Self::TypingStart),
            "USER_UPDATE" => Some(Self::UserUpdate),
            "VOICE_STATE_UPDATE" => Some(Self::VoiceStateUpdate),
            "VOICE_SERVER_UPDATE" => Some(Self::VoiceServerUpdate),
            "WEBHOOKS_UPDATE" => Some(Self::WebhooksUpdate),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    username: String,
    discriminator: Option<String>,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<u64>,
    premium_type: Option<u32>,
    public_flags: Option<u64>,
    global_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialApplication {
  id: String,
  flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    id: String,
    name: String,
    icon: Option<String>,
    description: String,
    rpc_origins: Option<Vec<String>>,
    bot_public: Option<bool>,
    bot_require_code_grant: Option<bool>,
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: Option<User>,
    summary: String,
    verify_key: String,
    team: Option<Team>,
    guild_id: Option<String>,
    primary_sku_id: Option<String>,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationUpdate {
  application_id: String,
  description: Option<String>,
  icon: Option<String>,
  cover_image: Option<String>,
  team_id: Option<String>,
  flags: Option<i64>,
  interactions_endpoint_url: Option<String>,
  max_participants: Option<i64>,
  #[serde(rename = "type")]
  application_type: Option<String>,
  tags: Vec<String>,
  custom_install_url: Option<String>,
  install_params: Option<String>,
  role_connections_verification_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMember {
    membership_state: u32,
    permissions: Vec<String>,
    team_id: String,
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    id: String,
    icon: Option<String>,
    members: Vec<TeamMember>,
    owner_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleTags {
    bot_id: Option<String>,
    integration_id: Option<String>,
    subscription_listing_id: Option<String>,
    premium_subscriber: Option<bool>, // will need to figure out how to handle
    available_for_purchase: Option<bool>, // will need to figure out how to handle
    guild_connections: Option<bool>, // will need to figure out how to handle

// premium_subscriber?	null	whether this is the guild's Booster role
// available_for_purchase?	null	whether this role is available for purchase
// guild_connections?	null	whether this role is a guild's linked role
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    id: String,
    name: String,
    color: u32,
    hoist: bool,
    icon: Option<String>,
    unicode_emoji: Option<String>,
    position: u64,
    permissions: String,
    managed: bool,
    mentionable: bool,
    tags: Option<RoleTags>,
    flags: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    id: Option<String>,
    name: Option<String>,
    roles: Option<Vec<String>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    avatar: Option<String>,
    roles: Vec<String>,
    joined_at: String,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool,
    flags: u64,
    pending: Option<bool>,
    permissions: Option<String>,
    guild_id: Option<String>,
    communication_disabled_until: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    id: String,
    #[serde(rename = "type")]
    channel_type: u32,
    guild_id: Option<String>,
    position: Option<u64>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<String>,
    bitrate: Option<u64>,
    user_limit: Option<u64>,
    rate_limit_per_user: Option<u64>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<String>,
    application_id: Option<String>,
    parent_id: Option<String>,
    last_pin_timestamp: Option<String>,
    rtc_region: Option<String>,
    video_quality_mode: Option<u32>,
    message_count: Option<u64>,
    member_count: Option<u64>,
    thread_metadata: Option<ThreadMetadata>,
    member: Option<ThreadMember>,
    default_auto_archive_duration: Option<u64>,
    permissions: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionOverwrite {
    id: String,
    #[serde(rename = "type")]
    overwrite_type: u32,
    allow: Option<u32>,
    deny: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMetadata {
    archived: bool,
    auto_archive_duration: u64,
    archive_timestamp: String,
    locked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMember {
    id: String,
    user_id: String,
    join_timestamp: String,
    flags: u32,
    guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresenceUpdate {
    user: User,
    guild_id: String,
    status: String,
    activities: Option<Vec<Activity>>,
    client_status: ClientStatus,
    premium_since: Option<String>,
    nick: Option<String>,
    roles: Option<Vec<String>>,
    guild_member: Option<GuildMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    activity_type: u32,
    url: Option<String>,
    created_at: u64,
    timestamps: Option<ActivityTimestamps>,
    application_id: Option<String>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<ActivityEmoji>,
    party: Option<ActivityParty>,
    assets: Option<ActivityAssets>,
    secrets: Option<ActivitySecrets>,
    instance: Option<bool>,
    flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityTimestamps {
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityEmoji {
    name: String,
    id: Option<u64>,
    animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityParty {
    id: Option<String>,
    size: Option<[u64; 2]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitySecrets {
    join: Option<String>,
    spectate: Option<String>,
    match_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientStatus {
    desktop: Option<String>,
    mobile: Option<String>,
    web: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeScreen {
    enabled: Option<bool>,
    description: Option<String>,
    welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeScreenChannel {
    channel_id: String,
    description: String,
    emoji_id: Option<String>,
    emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    id: String,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    owner: Option<bool>,
    owner_id: String,
    permissions: Option<u64>,
    region: String,
    afk_channel_id: Option<String>,
    afk_timeout: u64,
    widget_enabled: Option<bool>,
    widget_channel_id: Option<String>,
    verification_level: u32,
    default_message_notifications: u32,
    explicit_content_filter: u32,
    roles: Vec<Role>,
    emojis: Vec<Emoji>,
    features: Vec<String>,
    mfa_level: u32,
    application_id: Option<String>,
    system_channel_id: Option<String>,
    system_channel_flags: u32,
    rules_channel_id: Option<String>,
    max_presences: Option<u64>,
    max_members: Option<u64>,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_tier: u32,
    premium_subscription_count: Option<u64>,
    preferred_locale: String,
    public_updates_channel_id: Option<String>,
    max_video_channel_users: Option<u64>,
    approximate_member_count: Option<u64>,
    approximate_presence_count: Option<u64>,
    welcome_screen: Option<WelcomeScreen>,
    nsfw_level: u32,

    // May not be there on GuildCreate
    joined_at: Option<String>,
    large: Option<bool>,
    unavailable: Option<bool>,
    member_count: Option<u64>,
    voice_states: Option<Vec<VoiceState>>,
    members: Option<Vec<GuildMember>>,
    channels: Option<Vec<Channel>>,
    threads: Option<Vec<Channel>>,
    presences: Option<Vec<PresenceUpdate>>,
    stage_instances: Option<Vec<StageInstance>>,
    guild_scheduled_events: Option<Vec<GuildScheduledEvent>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnavailableGuild {
    id: String,
    unavailable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandPermissions {
    id: String,
    #[serde(rename = "type")]
    permission_type: u32,
    permission: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRule {
    id: String,
    name: String,
    enabled: bool,
    actions: Vec<AutoModerationAction>,
    conditions: Vec<AutoModerationCondition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationAction {
    #[serde(rename = "type")]
    action_type: u32,
    reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationCondition {
    #[serde(rename = "type")]
    condition_type: u32,
    #[serde(rename = "match")]
    match_: u32,
    #[serde(rename = "match_parameters")]
    match_parameters: Vec<String>,
}


// Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ready {
    // _trace: Vec<serde_json::Value>,
    pub application: PartialApplication,
    pub v: u32,
    pub user: User,
    pub guilds: Vec<serde_json::Value>, // Can be either a Guild or UnavailableGuild
    pub resume_gateway_url: String,
    pub session_id: String,
    pub shard: Option<[u64; 2]>,

    // Fields that shouldn't be here but are
    pub auth: Option<serde_json::Value>,
    pub current_location: Option<Vec<String>>,
    pub geo_ordered_rtc_regions: Option<Vec<String>>,
    pub guild_join_requests: Option<Vec<String>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub relationships: Option<Vec<String>>,
    pub private_channels: Option<Vec<String>>,
    pub session_type: Option<String>,
    pub user_settings: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandPermissionsUpdate {
    id: String,
    application_id: String,
    guild_id: String,
    permissions: Vec<ApplicationCommandPermissions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRuleCreate {
    guild_id: String,
    rule: AutoModerationRule,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRuleUpdate {
    guild_id: String,
    rule: AutoModerationRule,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRuleDelete {
    guild_id: String,
    rule_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationActionExecution {
    guild_id: String,
    rule_id: String,
    action: AutoModerationAction,
    user_id: String,
    reason: Option<String>,
    rule_trigger_type: u32,
    channel_id: Option<String>,
    message_id: Option<String>,
    alert_system_message_id: Option<String>,
    content: Option<String>,
    matched_keyword: Option<String>,
    matched_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelPinsUpdate {
    guild_id: Option<String>,
    channel_id: String,
    last_pin_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadDelete {
    id: String,
    guild_id: Option<String>,
    parent_id: Option<String>,
    #[serde(rename = "type")]
    channel_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadListSync {
    guild_id: String,
    channel_ids: Vec<String>,
    threads: Vec<Channel>,
    members: Vec<ThreadMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMembersUpdate {
    id: String,
    guild_id: Option<String>,
    member_count: u64,
    added_members: Vec<ThreadMember>,
    removed_member_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entitlement {
    id: String,
    sku_id: String,
    application_id: String,
    user_id: Option<String>,
    #[serde(rename = "type")]
    entitlement_type: u32,
    deleted: bool,
    starts_at: Option<String>,
    ends_at: Option<String>,
    guild_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditLogChange {
    key: String,
    new_value: Option<String>,
    old_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionalAuditEntryInfo {
    delete_member_days: Option<String>,
    members_removed: Option<String>,
    channel_id: Option<String>,
    message_id: Option<String>,
    count: Option<String>,
    id: Option<String>,
    #[serde(rename = "type")]
    #[serde(alias = "type")]
    audit_type: Option<String>,
    role_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditLogEntry {
    id: String,
    target_id: Option<String>,
    changes: Option<Vec<AuditLogChange>>,
    user_id: Option<String>,
    action_type: u32,
    options: Option<OptionalAuditEntryInfo>,
    reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildBanAdd {
    guild_id: String,
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildBanRemove {
    guild_id: String,
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildEmojisUpdate {
    guild_id: String,
    emojis: Vec<Emoji>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    id: String,
    pack_id: Option<String>,
    name: String,
    description: String,
    tags: Option<String>,
    asset: String,
    preview_asset: Option<String>,
    format_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildStickersUpdate {
    guild_id: String,
    stickers: Vec<Sticker>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildIntegrationsUpdate {
    guild_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMemberRemove {
    guild_id: String,
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMemberUpdate {
    guild_id: String,
    roles: Vec<String>,
    user: User,
    nick: Option<String>,
    avatar: Option<String>,
    joined_at: Option<String>,
    premium_since: Option<String>,
    deaf: Option<bool>,
    mute: Option<bool>,
    pending: Option<bool>,
    communication_disabled_until: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMembersChunk {
    guild_id: String,
    members: Vec<GuildMember>,
    chunk_index: u64,
    chunk_count: u64,
    not_found: Option<Vec<String>>,
    presences: Option<Vec<PresenceUpdate>>,
    nonce: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRoleCreate {
    guild_id: String,
    role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRoleUpdate {
    guild_id: String,
    role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRoleDelete {
    guild_id: String,
    role_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityMetadata {
    location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildScheduledEvent {
    id: String,
    guild_id: String,
    channel_id: Option<String>,
    creator_id: Option<String>,
    name: String,
    description: Option<String>,
    scheduled_start_time: String,
    scheduled_end_time: Option<String>,
    privacy_level: u32,
    status: u32,
    entity_type: u32,
    entity_id: Option<String>,
    entity_metadata: Option<EntityMetadata>,
    creator: Option<User>,
    user_count: Option<u64>,
    image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventUser {
  guild_scheduled_event_id: String,
  user_id: String,
  guild_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Integration {
    id: String,
    name: String,
    #[serde(rename = "type")]
    integration_type: String,
    enabled: bool,
    syncing: Option<bool>,
    role_id: Option<String>,
    enable_emoticons: Option<bool>,
    expire_behavior: Option<u32>,
    expire_grace_period: Option<u64>,
    user: Option<User>,
    account: Option<Account>,
    synced_at: Option<String>,
    subscriber_count: Option<u64>,
    revoked: Option<bool>,
    application: Option<Application>,
    scopes: Option<Vec<String>>,
    guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrationDelete {
    id: String,
    guild_id: String,
    application_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    id: String,
    filename: String,
    description: Option<String>,
    content_type: Option<String>,
    size: u64,
    url: String,
    proxy_url: String,
    height: Option<u64>,
    width: Option<u64>,
    ephemeral: Option<bool>,
    duration_secs: Option<f64>,
    waveform: Option<String>,
    flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    activity_type: u32,
    party_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReference {
    message_id: Option<String>,
    channel_id: String,
    guild_id: Option<String>,
    fail_if_not_exists: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
  count: u64,
  count_details: u64,
  me: bool,
  me_burst: bool,
  emoji: Emoji,
  burst_colors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<u32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedFooter {
  text: String,
  icon_url: Option<String>,
  proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedImage {
  url: String,
  proxy_url: Option<String>,
  height: Option<u64>,
  width: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedThumbnail {
  url: String,
  proxy_url: Option<String>,
  height: Option<u64>,
  width: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u64>,
    width: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedProvider {
  name: Option<String>,
  url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedAuthor {
  name: String,
  url: Option<String>,
  icon_url: Option<String>,
  proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedField {
  name: String,
  value: String,
  inline: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StickerItem {
  id: String,
  name: String,
  format_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleSubscriptionData {
  role_subscription_listing_id: String,
  tier_name: String,
  total_months_subscribed: u64,
  is_renewal: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: String,
    channel_id: String,
    author: User, // author*	user object	the author of this message (not guaranteed to be a valid user, see below)
    content: String,
    timestamp: String,
    edited_timestamp: Option<String>,
    tts: bool,
    mentions: Option<Vec<User>>,
    mention_everyone: bool,
    mention_roles: Vec<Role>,
    mention_channels: Option<Vec<Channel>>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<String>,
    pinned: bool,
    webhook_id: Option<String>,
    #[serde(rename = "type")]
    message_type: u32,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<String>,
    message_reference: Option<MessageReference>,
    flags: Option<u64>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<Box<Interaction>>,
    thread: Option<Channel>,
    components: Option<Vec<serde_json::Value>>, // Need to figure this one out
    sticker_items: Option<Vec<StickerItem>>,
    stickers: Option<Vec<Sticker>>,
    position: Option<u64>,
    role_subscription_data: Option<RoleSubscriptionData>,
    resolved: Option<ResolvedData>,
    guild_id: Option<String>,
    member: Option<GuildMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolvedData {
  users: Option<HashMap<String, User>>,
  members: Option<HashMap<String, GuildMember>>,
  roles: Option<HashMap<String, Role>>,
  channels: Option<HashMap<String, Channel>>,
  messages: Option<HashMap<String, Message>>,
  attachments: Option<HashMap<String, Attachment>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionDataOption {
  name: String,
  #[serde(rename = "type")]
  option_type: u32,
  value: Option<serde_json::Value>,
  options: Option<Vec<ApplicationCommandInteractionDataOption>>,
  focused: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionData {
  id: String,
  name: String,
  #[serde(rename = "type")]
  interaction_type: u32,
  resolved: Option<ResolvedData>,
  options: Option<Vec<ApplicationCommandInteractionDataOption>>,
  guild_id: Option<String>,
  target_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
  id: String,
  application_id: String,
  #[serde(rename = "type")]
  interaction_type: u32,
  data: Option<InteractionData>,
  guild_id: Option<String>,
  channel_id: Option<String>,
  member: Option<GuildMember>,
  user: Option<User>,
  token: String,
  version: u32,
  message: Option<Message>,
  app_permissions: Option<String>,
  locale: Option<String>,
  guild_locale: Option<String>,
  entitlements: Option<Vec<Entitlement>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteCreate {
  channel_id: String,
  code: String,
  created_at: String,
  guild_id: Option<String>,
  inviter: Option<User>,
  max_age: Option<u64>,
  max_uses: Option<u64>,
  target_type: Option<u32>,
  target_user: Option<User>,
  target_application: Option<Application>,
  temporary: Option<bool>,
  uses: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteDelete {
  channel_id: String,
  guild_id: Option<String>,
  code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDelete {
  id: String,
  channel_id: String,
  guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDeleteBulk {
  ids: Vec<String>,
  channel_id: String,
  guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionAdd {
  user_id: String,
  channel_id: String,
  message_id: String,
  guild_id: Option<String>,
  member: Option<GuildMember>,
  emoji: Emoji,
  message_author_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionRemove {
  user_id: String,
  channel_id: String,
  message_id: String,
  guild_id: Option<String>,
  emoji: Emoji,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionRemoveAll {
  channel_id: String,
  message_id: String,
  guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionRemoveEmoji {
  channel_id: String,
  guild_id: Option<String>,
  message_id: String,
  emoji: Emoji,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypingStart {
  channel_id: String,
  guild_id: Option<String>,
  user_id: String,
  timestamp: u64,
  member: Option<GuildMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceState {
  guild_id: Option<String>,
  channel_id: Option<String>,
  user_id: String,
  member: Option<GuildMember>,
  session_id: String,
  deaf: bool,
  mute: bool,
  self_deaf: bool,
  self_mute: bool,
  self_stream: Option<bool>,
  self_video: bool,
  suppress: bool,
  request_to_speak_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceServerUpdate {
  token: String,
  guild_id: String,
  endpoint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhooksUpdate {
  guild_id: String,
  channel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StageInstance {
  id: String,
  guild_id: String,
  channel_id: String,
  topic: String,
  privacy_level: u32,
  discoverable_disabled: bool,
  guild_scheduled_event_id: Option<String>,
}

// HTTP API:
// https://discord.com/api/v9
// Must add User-Agent: DiscordBot ($url, $versionNumber)
// Authorization: Bot MTk4NjIyNDgzNDcxOTI1MjQ4.Cl2FMQ.ZnCjm1XVW7vRze4b7Cq4se7kKWs

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildChannelUpdate {
  id: String,
  position: Option<u32>,
  parent_id: Option<String>,
  lock_permissions: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildChannel {
  name: String,
  #[serde(rename = "type")]
  channel_type: Option<String>,
  position: Option<i32>,
  topic: Option<String>,
  bitrate: Option<i32>,
  user_limit: Option<i32>,
  nsfw: Option<bool>,
  rate_limit_per_user: Option<i32>,
  parent_id: Option<String>,
  permission_overwrites: Vec<PermissionOverwrite>,
  rtc_region: Option<String>,
  video_quality_mode: Option<String>,
  default_auto_archive_duration: Option<String>,
  default_reaction_emoji: Option<String>,
  default_sort_order: Option<String>,
  default_forum_layout: Option<String>,
  available_tags: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionType {
  title: String,
  #[serde(rename = "const")]
  permission_const: u32,
  description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandPermission {
  id: String,
  #[serde(rename = "type")]
  permission_type: PermissionType,
  permission: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    option_type: u32,
    name: String,
    description: String,
    name_localizations: Option<HashMap<String, String>>,
    description_localizations: Option<HashMap<String, String>>,
    required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildApplicationCommand {
  application_id: String,
  guild_id: String,
  command_id: String,
  name: String,
  name_localizations: Option<HashMap<String, String>>,
  description: Option<String>,
  description_localizations: Option<HashMap<String, String>>,
  default_member_permissions: Option<u32>,
  dm_permission: Option<bool>,
  options: Vec<ApplicationCommandOption>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRole {
  id: String,
  name: Option<String>,
  color: Option<u32>,
  permissions: Option<u32>,
  hoist: Option<bool>,
  mentionable: Option<bool>,
  unicode_emoji: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewGuild {
	name: String,
	description: Option<String>,
	region: Option<String>,
	icon: Option<String>,
	verification_level: Option<u32>,
	default_message_notifications: Option<u32>,
	explicit_content_filter: Option<u32>,
	preferred_locale: Option<String>,
	afk_timeout: Option<u32>,
	roles: Vec<GuildRole>,
	channels: Option<Vec<GuildChannel>>,
	afk_channel_id: Option<String>,
	system_channel_id: Option<String>,
	system_channel_flags: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGuild {
    name: Option<String>,
    description: Option<String>,
    region: Option<String>,
    icon: Option<String>,
    verification_level: Option<String>,
    default_message_notifications: Option<String>,
    explicit_content_filter: Option<String>,
    preferred_locale: Option<String>,
    afk_timeout: Option<String>,
    afk_channel_id: Option<String>,
    system_channel_id: Option<String>,
    owner_id: Option<String>,
    splash: Option<String>,
    banner: Option<String>,
    system_channel_flags: Option<String>,
    features: Option<Vec<String>>,
    discovery_splash: Option<String>,
    home_header: Option<String>,
    rules_channel_id: Option<String>,
    safety_alerts_channel_id: Option<String>,
    public_updates_channel_id: Option<String>,
    premium_progress_bar_enabled: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowedMention {
  parse: Vec<String>,
  roles: Vec<String>,
  users: Vec<String>,
  replied_user: bool,
}

#[derive(Serialize, Deserialize, Debug)]
  pub struct InteractionCallbackData {
      tts: Option<bool>,
      content: Option<String>,
      embeds: Option<Vec<Embed>>,
      allowed_mentions: Option<AllowedMention>,
      flags: Option<u32>,
      components: Option<Vec<serde_json::Value>>,
      attachments: Option<Vec<Attachment>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookMessage {
  content: Option<String>,
  embeds: Option<Vec<Embed>>,
  allowed_mentions: Option<AllowedMention>,
  attachments: Option<Vec<Attachment>>,
  components: Option<Vec<serde_json::Value>>,
  payload_json: Option<String>,
  flags: Option<u64>,
  // files[n] **	file contents	the contents of the file being sent/edited // REQUIRES CHANGING TO FORM DATA
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpGuildRoleUpdate {
  id: Option<String>,
  position: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationRoleConnectionsMetadata {
  #[serde(rename = "type")]
  application_role_connections_metadata_type: PermissionType,
  key: String,
  name: String,
  description: String,
  name_localizations: HashMap<String, String>,
  description_localizations: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpStageInstance {
  topic: String,
  channel_id: String,
  privacy_level: Option<u32>,
  guild_scheduled_event_id: Option<String>,
  send_start_notification: Option<bool>,
}
