use super::types::*;

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

pub fn parse_gateway_payload(payload_bytes: &[u8], seq: &mut u64) -> anyhow::Result<GatewayReceiveEvent> {
  let payload  = match serde_json::from_slice::<GatewayPayload>(payload_bytes) {
      Ok(payload) => payload,
      Err(_) => {
        print_to_terminal(0, &format!("discord_api: not a valid GatewayPayload {}", payload_bytes.to_vec().into_iter().map(|x| x as char).collect::<String>()));
        return Err(anyhow::anyhow!("Failed to parse gateway payload"));
      }
  };

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
