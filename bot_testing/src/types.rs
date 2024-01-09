use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uqbar_process_lib::print_to_terminal;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectGateway {
    pub bot_token: String,
    pub intents: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayPayload {
    op: u8,
    d: serde_json::Value,
    s: Option<u64>,
    t: Option<String>,
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
    activity_type: u8,
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
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "READY" => Some(Self::Ready),
            "APPLICATIONCOMMANDPERMISSIONSUPDATE" => Some(Self::ApplicationCommandPermissionsUpdate),
            "AUTOMODERATIONRULECREATE" => Some(Self::AutoModerationRuleCreate),
            "AUTOMODERATIONRULEUPDATE" => Some(Self::AutoModerationRuleUpdate),
            "AUTOMODERATIONRULEDELETE" => Some(Self::AutoModerationRuleDelete),
            "AUTOMODERATIONACTIONEXECUTION" => Some(Self::AutoModerationActionExecution),
            "CHANNELCREATE" => Some(Self::ChannelCreate),
            "CHANNELUPDATE" => Some(Self::ChannelUpdate),
            "CHANNELDELETE" => Some(Self::ChannelDelete),
            "CHANNELPINSUPDATE" => Some(Self::ChannelPinsUpdate),
            "THREADCREATE" => Some(Self::ThreadCreate),
            "THREADUPDATE" => Some(Self::ThreadUpdate),
            "THREADDELETE" => Some(Self::ThreadDelete),
            "THREADLISTSYNC" => Some(Self::ThreadListSync),
            "THREADMEMBERUPDATE" => Some(Self::ThreadMemberUpdate),
            "THREADMEMBERSUPDATE" => Some(Self::ThreadMembersUpdate),
            "ENTITLEMENTCREATE" => Some(Self::EntitlementCreate),
            "ENTITLEMENTUPDATE" => Some(Self::EntitlementUpdate),
            "ENTITLEMENTDELETE" => Some(Self::EntitlementDelete),
            "GUILDCREATE" => Some(Self::GuildCreate),
            "GUILDUPDATE" => Some(Self::GuildUpdate),
            "GUILDDELETE" => Some(Self::GuildDelete),
            "GUILDAUDITLOGENTRYCREATE" => Some(Self::GuildAuditLogEntryCreate),
            "GUILDBANADD" => Some(Self::GuildBanAdd),
            "GUILDBANREMOVE" => Some(Self::GuildBanRemove),
            "GUILDEMOJISUPDATE" => Some(Self::GuildEmojisUpdate),
            "GUILDSTICKERSUPDATE" => Some(Self::GuildStickersUpdate),
            "GUILDINTEGRATIONSUPDATE" => Some(Self::GuildIntegrationsUpdate),
            "GUILDMEMBERADD" => Some(Self::GuildMemberAdd),
            "GUILDMEMBERREMOVE" => Some(Self::GuildMemberRemove),
            "GUILDMEMBERUPDATE" => Some(Self::GuildMemberUpdate),
            "GUILDMEMBERSCHUNK" => Some(Self::GuildMembersChunk),
            "GUILDROLECREATE" => Some(Self::GuildRoleCreate),
            "GUILDROLEUPDATE" => Some(Self::GuildRoleUpdate),
            "GUILDROLEDELETE" => Some(Self::GuildRoleDelete),
            "GUILDSCHEDULEDEVENTCREATE" => Some(Self::GuildScheduledEventCreate),
            "GUILDSCHEDULEDEVENTUPDATE" => Some(Self::GuildScheduledEventUpdate),
            "GUILDSCHEDULEDEVENTDELETE" => Some(Self::GuildScheduledEventDelete),
            "GUILDSCHEDULEDEVENTUSERADD" => Some(Self::GuildScheduledEventUserAdd),
            "GUILDSCHEDULEDEVENTUSERREMOVE" => Some(Self::GuildScheduledEventUserRemove),
            "INTEGRATIONCREATE" => Some(Self::IntegrationCreate),
            "INTEGRATIONUPDATE" => Some(Self::IntegrationUpdate),
            "INTEGRATIONDELETE" => Some(Self::IntegrationDelete),
            "INTERACTIONCREATE" => Some(Self::InteractionCreate),
            "INVITECREATE" => Some(Self::InviteCreate),
            "INVITEDELETE" => Some(Self::InviteDelete),
            "MESSAGECREATE" => Some(Self::MessageCreate),
            "MESSAGEUPDATE" => Some(Self::MessageUpdate),
            "MESSAGEDELETE" => Some(Self::MessageDelete),
            "MESSAGEDELETEBULK" => Some(Self::MessageDeleteBulk),
            "MESSAGEREACTIONADD" => Some(Self::MessageReactionAdd),
            "MESSAGEREACTIONREMOVE" => Some(Self::MessageReactionRemove),
            "MESSAGEREACTIONREMOVEALL" => Some(Self::MessageReactionRemoveAll),
            "MESSAGEREACTIONREMOVEEMOJI" => Some(Self::MessageReactionRemoveEmoji),
            "PRESENCEUPDATE" => Some(Self::PresenceUpdate),
            "STAGEINSTANCECREATE" => Some(Self::StageInstanceCreate),
            "STAGEINSTANCEUPDATE" => Some(Self::StageInstanceUpdate),
            "STAGEINSTANCEDELETE" => Some(Self::StageInstanceDelete),
            "TYPINGSTART" => Some(Self::TypingStart),
            "USERUPDATE" => Some(Self::UserUpdate),
            "VOICESTATEUPDATE" => Some(Self::VoiceStateUpdate),
            "VOICESERVERUPDATE" => Some(Self::VoiceServerUpdate),
            "WEBHOOKSUPDATE" => Some(Self::WebhooksUpdate),
            _ => None,
        }
    }
}

pub fn parse_gateway_payload(payload_bytes: &[u8], seq: &mut u64) -> anyhow::Result<GatewayReceiveEvent> {
    let payload  = serde_json::from_slice::<GatewayPayload>(payload_bytes)?;

    if let Some(s) = payload.s {
        *seq = s;
    }

    match payload.op {
        1 => {
            return Ok(GatewayReceiveEvent::Heartbeat);
        },
        7 => {
            return Ok(GatewayReceiveEvent::Reconnect);
        },
        9 => {
            let resumable = serde_json::from_value::<bool>(payload.d.clone())?;
            return Ok(GatewayReceiveEvent::InvalidSession(resumable));
        },
        10 => {
          let data = match serde_json::from_value::<Hello>(payload.d.clone()) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data {}", "Hello", payload.d.clone().to_string()))
            }
          };
          return Ok(GatewayReceiveEvent::Hello(data));
        },
        11 => {
            return Ok(GatewayReceiveEvent::HeartbeatAck);
        },
        _ => {},
    }

    let Some(t) = GatewayEventType::from_str(&payload.t.clone().unwrap_or("".to_string())) else {
        return Err(anyhow::anyhow!("Unknown event type: {}", payload.t.unwrap_or("".to_string())));
    };
    // print_to_terminal(0, &format!("discord_api: 0.3"));

    let payload: GatewayReceiveEvent = match t {
        GatewayEventType::Ready => {
          let data = match serde_json::from_value::<Ready>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "Ready"))
            }
          };
          GatewayReceiveEvent::Ready(data)
        }
        GatewayEventType::Resumed => GatewayReceiveEvent::Resumed,
        GatewayEventType::ApplicationCommandPermissionsUpdate => {
          let data = match serde_json::from_value::<ApplicationCommandPermissionsUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ApplicationCommandPermissionsUpdate"))
            }
          };
          GatewayReceiveEvent::ApplicationCommandPermissionsUpdate(data)
        }
        GatewayEventType::AutoModerationRuleCreate => {
          let data = match serde_json::from_value::<AutoModerationRuleCreate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "AutoModerationRuleCreate"))
            }
          };
          GatewayReceiveEvent::AutoModerationRuleCreate(data)
        }
        GatewayEventType::AutoModerationRuleUpdate => {
          let data = match serde_json::from_value::<AutoModerationRuleUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "AutoModerationRuleUpdate"))
            }
          };
          GatewayReceiveEvent::AutoModerationRuleUpdate(data)
        }
        GatewayEventType::AutoModerationRuleDelete => {
          let data = match serde_json::from_value::<AutoModerationRuleDelete>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "AutoModerationRuleDelete"))
            }
          };
          GatewayReceiveEvent::AutoModerationRuleDelete(data)
        }
        GatewayEventType::AutoModerationActionExecution => {
          let data = match serde_json::from_value::<AutoModerationActionExecution>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "AutoModerationActionExecution"))
            }
          };
          GatewayReceiveEvent::AutoModerationActionExecution(data)
        }
        GatewayEventType::ChannelCreate => {
          let data = match serde_json::from_value::<Channel>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ChannelCreate"))
            }
          };
          GatewayReceiveEvent::ChannelCreate(data)
        }
        GatewayEventType::ChannelUpdate => {
          let data = match serde_json::from_value::<Channel>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ChannelUpdate"))
            }
          };
          GatewayReceiveEvent::ChannelUpdate(data)
        }
        GatewayEventType::ChannelDelete => {
          let data = match serde_json::from_value::<Channel>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ChannelDelete"))
            }
          };
          GatewayReceiveEvent::ChannelDelete(data)
        }
        GatewayEventType::ChannelPinsUpdate => {
          let data = match serde_json::from_value::<ChannelPinsUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ChannelPinsUpdate"))
            }
          };
          GatewayReceiveEvent::ChannelPinsUpdate(data)
        }
        GatewayEventType::ThreadCreate => {
          let data = match serde_json::from_value::<Channel>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ThreadCreate"))
            }
          };
          GatewayReceiveEvent::ThreadCreate(data)
        }
        GatewayEventType::ThreadUpdate => {
          let data = match serde_json::from_value::<Channel>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ThreadUpdate"))
            }
          };
          GatewayReceiveEvent::ThreadUpdate(data)
        }
        GatewayEventType::ThreadDelete => {
          let data = match serde_json::from_value::<ThreadDelete>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ThreadDelete"))
            }
          };
          GatewayReceiveEvent::ThreadDelete(data)
        }
        GatewayEventType::ThreadListSync => {
          let data = match serde_json::from_value::<ThreadListSync>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ThreadListSync"))
            }
          };
          GatewayReceiveEvent::ThreadListSync(data)
        }
        GatewayEventType::ThreadMemberUpdate => {
          let data = match serde_json::from_value::<ThreadMember>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ThreadMemberUpdate"))
            }
          };
          GatewayReceiveEvent::ThreadMemberUpdate(data)
        }
        GatewayEventType::ThreadMembersUpdate => {
          let data = match serde_json::from_value::<ThreadMembersUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "ThreadMembersUpdate"))
            }
          };
          GatewayReceiveEvent::ThreadMembersUpdate(data)
        }
        GatewayEventType::EntitlementCreate => {
          let data = match serde_json::from_value::<Entitlement>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "EntitlementCreate"))
            }
          };
          GatewayReceiveEvent::EntitlementCreate(data)
        }
        GatewayEventType::EntitlementUpdate => {
          let data = match serde_json::from_value::<Entitlement>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "EntitlementUpdate"))
            }
          };
          GatewayReceiveEvent::EntitlementUpdate(data)
        }
        GatewayEventType::EntitlementDelete => {
          let data = match serde_json::from_value::<Entitlement>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "EntitlementDelete"))
            }
          };
          GatewayReceiveEvent::EntitlementDelete(data)
        }
        GatewayEventType::GuildCreate => {
          let data = match serde_json::from_value::<Guild>(payload.d.clone()) {
            Ok(data) => (Some(data), None),
            Err(_) => {
              match serde_json::from_value::<UnavailableGuild>(payload.d) {
                Ok(data) => (None, Some(data)),
                Err(_) => {
                  return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildCreate"))
                }
              }
            }
          };
          GatewayReceiveEvent::GuildCreate(data)
        }
        GatewayEventType::GuildUpdate => {
          let data = match serde_json::from_value::<Guild>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildUpdate"))
            }
          };
          GatewayReceiveEvent::GuildUpdate(data)
        }
        GatewayEventType::GuildDelete => {
          let data = match serde_json::from_value::<UnavailableGuild>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildDelete"))
            }
          };
          GatewayReceiveEvent::GuildDelete(data)
        }
        GatewayEventType::GuildAuditLogEntryCreate => {
          let data = match serde_json::from_value::<AuditLogEntry>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildAuditLogEntryCreate"))
            }
          };
          GatewayReceiveEvent::GuildAuditLogEntryCreate(data)
        }
        GatewayEventType::GuildBanAdd => {
          let data = match serde_json::from_value::<GuildBanAdd>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildBanAdd"))
            }
          };
          GatewayReceiveEvent::GuildBanAdd(data)
        }
        GatewayEventType::GuildBanRemove => {
          let data = match serde_json::from_value::<GuildBanRemove>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildBanRemove"))
            }
          };
          GatewayReceiveEvent::GuildBanRemove(data)
        }
        GatewayEventType::GuildEmojisUpdate => {
          let data = match serde_json::from_value::<GuildEmojisUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildEmojisUpdate"))
            }
          };
          GatewayReceiveEvent::GuildEmojisUpdate(data)
        }
        GatewayEventType::GuildStickersUpdate => {
          let data = match serde_json::from_value::<GuildStickersUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildStickersUpdate"))
            }
          };
          GatewayReceiveEvent::GuildStickersUpdate(data)
        }
        GatewayEventType::GuildIntegrationsUpdate => {
          let data = match serde_json::from_value::<GuildIntegrationsUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildIntegrationsUpdate"))
            }
          };
          GatewayReceiveEvent::GuildIntegrationsUpdate(data)
        }
        GatewayEventType::GuildMemberAdd => {
          let data = match serde_json::from_value::<GuildMember>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildMemberAdd"))
            }
          };
          GatewayReceiveEvent::GuildMemberAdd(data)
        }
        GatewayEventType::GuildMemberRemove => {
          let data = match serde_json::from_value::<GuildMemberRemove>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildMemberRemove"))
            }
          };
          GatewayReceiveEvent::GuildMemberRemove(data)
        }
        GatewayEventType::GuildMemberUpdate => {
          let data = match serde_json::from_value::<GuildMemberUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildMemberUpdate"))
            }
          };
          GatewayReceiveEvent::GuildMemberUpdate(data)
        }
        GatewayEventType::GuildMembersChunk => {
          let data = match serde_json::from_value::<GuildMembersChunk>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildMembersChunk"))
            }
          };
          GatewayReceiveEvent::GuildMembersChunk(data)
        }
        GatewayEventType::GuildRoleCreate => {
          let data = match serde_json::from_value::<GuildRoleCreate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildRoleCreate"))
            }
          };
          GatewayReceiveEvent::GuildRoleCreate(data)
        }
        GatewayEventType::GuildRoleUpdate => {
          let data = match serde_json::from_value::<GuildRoleUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildRoleUpdate"))
            }
          };
          GatewayReceiveEvent::GuildRoleUpdate(data)
        }
        GatewayEventType::GuildRoleDelete => {
          let data = match serde_json::from_value::<GuildRoleDelete>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildRoleDelete"))
            }
          };
          GatewayReceiveEvent::GuildRoleDelete(data)
        }
        GatewayEventType::GuildScheduledEventCreate => {
          let data = match serde_json::from_value::<GuildScheduledEvent>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildScheduledEventCreate"))
            }
          };
          GatewayReceiveEvent::GuildScheduledEventCreate(data)
        }
        GatewayEventType::GuildScheduledEventUpdate => {
          let data = match serde_json::from_value::<GuildScheduledEvent>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildScheduledEventUpdate"))
            }
          };
          GatewayReceiveEvent::GuildScheduledEventUpdate(data)
        }
        GatewayEventType::GuildScheduledEventDelete => {
          let data = match serde_json::from_value::<GuildScheduledEvent>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildScheduledEventDelete"))
            }
          };
          GatewayReceiveEvent::GuildScheduledEventDelete(data)
        }
        GatewayEventType::GuildScheduledEventUserAdd => {
          let data = match serde_json::from_value::<GuildScheduledEventUser>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildScheduledEventUserAdd"))
            }
          };
          GatewayReceiveEvent::GuildScheduledEventUserAdd(data)
        }
        GatewayEventType::GuildScheduledEventUserRemove => {
          let data = match serde_json::from_value::<GuildScheduledEventUser>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "GuildScheduledEventUserRemove"))
            }
          };
          GatewayReceiveEvent::GuildScheduledEventUserRemove(data)
        }
        GatewayEventType::IntegrationCreate => {
          let data = match serde_json::from_value::<Integration>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "IntegrationCreate"))
            }
          };
          GatewayReceiveEvent::IntegrationCreate(data)
        }
        GatewayEventType::IntegrationUpdate => {
          let data = match serde_json::from_value::<Integration>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "IntegrationUpdate"))
            }
          };
          GatewayReceiveEvent::IntegrationUpdate(data)
        }
        GatewayEventType::IntegrationDelete => {
          let data = match serde_json::from_value::<IntegrationDelete>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "IntegrationDelete"))
            }
          };
          GatewayReceiveEvent::IntegrationDelete(data)
        }
        GatewayEventType::InteractionCreate => {
          let data = match serde_json::from_value::<Interaction>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "InteractionCreate"))
            }
          };
          GatewayReceiveEvent::InteractionCreate(data)
        }
        GatewayEventType::InviteCreate => {
          let data = match serde_json::from_value::<InviteCreate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "InviteCreate"))
            }
          };
          GatewayReceiveEvent::InviteCreate(data)
        }
        GatewayEventType::InviteDelete => {
          let data = match serde_json::from_value::<InviteDelete>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "InviteDelete"))
            }
          };
          GatewayReceiveEvent::InviteDelete(data)
        }
        GatewayEventType::MessageCreate => {
          let data = match serde_json::from_value::<Message>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageCreate"))
            }
          };
          GatewayReceiveEvent::MessageCreate(data)
        }
        GatewayEventType::MessageUpdate => {
          let data = match serde_json::from_value::<Message>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageUpdate"))
            }
          };
          GatewayReceiveEvent::MessageUpdate(data)
        }
        GatewayEventType::MessageDelete => {
          let data = match serde_json::from_value::<MessageDelete>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageDelete"))
            }
          };
          GatewayReceiveEvent::MessageDelete(data)
        }
        GatewayEventType::MessageDeleteBulk => {
          let data = match serde_json::from_value::<MessageDeleteBulk>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageDeleteBulk"))
            }
          };
          GatewayReceiveEvent::MessageDeleteBulk(data)
        }
        GatewayEventType::MessageReactionAdd => {
          let data = match serde_json::from_value::<MessageReactionAdd>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageReactionAdd"))
            }
          };
          GatewayReceiveEvent::MessageReactionAdd(data)
        }
        GatewayEventType::MessageReactionRemove => {
          let data = match serde_json::from_value::<MessageReactionRemove>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageReactionRemove"))
            }
          };
          GatewayReceiveEvent::MessageReactionRemove(data)
        }
        GatewayEventType::MessageReactionRemoveAll => {
          let data = match serde_json::from_value::<MessageReactionRemoveAll>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageReactionRemoveAll"))
            }
          };
          GatewayReceiveEvent::MessageReactionRemoveAll(data)
        }
        GatewayEventType::MessageReactionRemoveEmoji => {
          let data = match serde_json::from_value::<MessageReactionRemoveEmoji>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "MessageReactionRemoveEmoji"))
            }
          };
          GatewayReceiveEvent::MessageReactionRemoveEmoji(data)
        }
        GatewayEventType::PresenceUpdate => {
          let data = match serde_json::from_value::<PresenceUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "PresenceUpdate"))
            }
          };
          GatewayReceiveEvent::PresenceUpdate(data)
        }
        GatewayEventType::StageInstanceCreate => {
          let data = match serde_json::from_value::<StageInstance>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "StageInstanceCreate"))
            }
          };
          GatewayReceiveEvent::StageInstanceCreate(data)
        }
        GatewayEventType::StageInstanceUpdate => {
          let data = match serde_json::from_value::<StageInstance>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "StageInstanceUpdate"))
            }
          };
          GatewayReceiveEvent::StageInstanceUpdate(data)
        }
        GatewayEventType::StageInstanceDelete => {
          let data = match serde_json::from_value::<StageInstance>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "StageInstanceDelete"))
            }
          };
          GatewayReceiveEvent::StageInstanceDelete(data)
        }
        GatewayEventType::TypingStart => {
          let data = match serde_json::from_value::<TypingStart>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "TypingStart"))
            }
          };
          GatewayReceiveEvent::TypingStart(data)
        }
        GatewayEventType::UserUpdate => {
          let data = match serde_json::from_value::<User>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "UserUpdate"))
            }
          };
          GatewayReceiveEvent::UserUpdate(data)
        }
        GatewayEventType::VoiceStateUpdate => {
          let data = match serde_json::from_value::<VoiceState>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "VoiceStateUpdate"))
            }
          };
          GatewayReceiveEvent::VoiceStateUpdate(data)
        }
        GatewayEventType::VoiceServerUpdate => {
          let data = match serde_json::from_value::<VoiceServerUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "VoiceServerUpdate"))
            }
          };
          GatewayReceiveEvent::VoiceServerUpdate(data)
        }
        GatewayEventType::WebhooksUpdate => {
          let data = match serde_json::from_value::<WebhooksUpdate>(payload.d) {
            Ok(data) => data,
            Err(_) => {
              return Err(anyhow::anyhow!("Failed to parse {} event with data", "WebhooksUpdate"))
            }
          };
          GatewayReceiveEvent::WebhooksUpdate(data)
        }
    };

    Ok(payload)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<u64>,
    premium_type: Option<u8>,
    public_flags: Option<u64>,
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
pub struct TeamMember {
    membership_state: u8,
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
    channel_type: u8,
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
    video_quality_mode: Option<u8>,
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
    overwrite_type: u8,
    allow: String,
    deny: String,
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
    flags: u8,
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
    activity_type: u8,
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
    verification_level: u8,
    default_message_notifications: u8,
    explicit_content_filter: u8,
    roles: Vec<Role>,
    emojis: Vec<Emoji>,
    features: Vec<String>,
    mfa_level: u8,
    application_id: Option<String>,
    system_channel_id: Option<String>,
    system_channel_flags: u8,
    rules_channel_id: Option<String>,
    max_presences: Option<u64>,
    max_members: Option<u64>,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_tier: u8,
    premium_subscription_count: Option<u64>,
    preferred_locale: String,
    public_updates_channel_id: Option<String>,
    max_video_channel_users: Option<u64>,
    approximate_member_count: Option<u64>,
    approximate_presence_count: Option<u64>,
    welcome_screen: Option<WelcomeScreen>,
    nsfw_level: u8,

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
    permission_type: u8,
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
    action_type: u8,
    reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationCondition {
    #[serde(rename = "type")]
    condition_type: u8,
    #[serde(rename = "match")]
    match_: u8,
    #[serde(rename = "match_parameters")]
    match_parameters: Vec<String>,
}
















#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ready {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<Guild>,
    pub resume_gateway_url: String,
    pub session_id: String,
    pub shard: Option<[u64; 2]>,
    pub application: PartialApplication,
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
    rule_trigger_type: u8,
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
    channel_type: u8,
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
    entitlement_type: u8,
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
    action_type: u8,
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
    format_type: u8,
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
    privacy_level: u8,
    status: u8,
    entity_type: u8,
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
    expire_behavior: Option<u8>,
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
    activity_type: u8,
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
  format_type: u8,
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
    message_type: u8,
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
  option_type: u8,
  value: Option<serde_json::Value>,
  options: Option<Vec<ApplicationCommandInteractionDataOption>>,
  focused: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionData {
  id: String,
  name: String,
  #[serde(rename = "type")]
  interaction_type: u8,
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
  interaction_type: u8,
  data: Option<InteractionData>,
  guild_id: Option<String>,
  channel_id: Option<String>,
  member: Option<GuildMember>,
  user: Option<User>,
  token: String,
  version: u8,
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
  target_type: Option<u8>,
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
  privacy_level: u8,
  discoverable_disabled: bool,
  guild_scheduled_event_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewayReceiveEvent {
  Hello(Hello),
  Ready(Ready),
  Resumed,
  Reconnect,
  InvalidSession(bool),
  Heartbeat,
  HeartbeatAck,
  ApplicationCommandPermissionsUpdate(ApplicationCommandPermissionsUpdate),
  AutoModerationRuleCreate(AutoModerationRuleCreate),
  AutoModerationRuleUpdate(AutoModerationRuleUpdate),
  AutoModerationRuleDelete(AutoModerationRuleDelete),
  AutoModerationActionExecution(AutoModerationActionExecution),
  ChannelCreate(Channel),
  ChannelUpdate(Channel),
  ChannelDelete(Channel),
  ThreadCreate(Channel),
  ThreadUpdate(Channel),
  ThreadDelete(ThreadDelete),
  ThreadListSync(ThreadListSync),
  ThreadMemberUpdate(ThreadMember),
  ThreadMembersUpdate(ThreadMembersUpdate),
  ChannelPinsUpdate(ChannelPinsUpdate),
  EntitlementCreate(Entitlement),
  EntitlementUpdate(Entitlement),
  EntitlementDelete(Entitlement),
  GuildCreate((Option<Guild>, Option<UnavailableGuild>)),
  GuildUpdate(Guild),
  GuildDelete(UnavailableGuild),
  GuildAuditLogEntryCreate(AuditLogEntry),
  GuildBanAdd(GuildBanAdd),
  GuildBanRemove(GuildBanRemove),
  GuildEmojisUpdate(GuildEmojisUpdate),
  GuildStickersUpdate(GuildStickersUpdate),
  GuildIntegrationsUpdate(GuildIntegrationsUpdate),
  GuildMemberAdd(GuildMember),
  GuildMemberRemove(GuildMemberRemove),
  GuildMemberUpdate(GuildMemberUpdate),
  GuildMembersChunk(GuildMembersChunk),
  GuildRoleCreate(GuildRoleCreate),
  GuildRoleUpdate(GuildRoleUpdate),
  GuildRoleDelete(GuildRoleDelete),
  GuildScheduledEventCreate(GuildScheduledEvent),
  GuildScheduledEventUpdate(GuildScheduledEvent),
  GuildScheduledEventDelete(GuildScheduledEvent),
  GuildScheduledEventUserAdd(GuildScheduledEventUser),
  GuildScheduledEventUserRemove(GuildScheduledEventUser),
  IntegrationCreate(Integration),
  IntegrationUpdate(Integration),
  IntegrationDelete(IntegrationDelete),
  InteractionCreate(Interaction),
  InviteCreate(InviteCreate),
  InviteDelete(InviteDelete),
  MessageCreate(Message),
  MessageUpdate(Message),
  MessageDelete(MessageDelete),
  MessageDeleteBulk(MessageDeleteBulk),
  MessageReactionAdd(MessageReactionAdd),
  MessageReactionRemove(MessageReactionRemove),
  MessageReactionRemoveAll(MessageReactionRemoveAll),
  MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
  PresenceUpdate(PresenceUpdate),
  StageInstanceCreate(StageInstance),
  StageInstanceUpdate(StageInstance),
  StageInstanceDelete(StageInstance),
  TypingStart(TypingStart),
  UserUpdate(User),
  VoiceStateUpdate(VoiceState),
  VoiceServerUpdate(VoiceServerUpdate),
  WebhooksUpdate(WebhooksUpdate),
}
