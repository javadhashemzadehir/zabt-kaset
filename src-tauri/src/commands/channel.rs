use crate::channel::{self, ChannelSyncPlan, ChannelSyncRequest};

#[tauri::command]
pub fn channel_plan_sync(request: ChannelSyncRequest) -> Result<ChannelSyncPlan, String> {
    channel::plan_sync(&request).map_err(|e| e.to_command_error())
}
