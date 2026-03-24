use core_lib::transfer::Direction;
use core_lib::{
    Active, ClientData, ClientId, GetFirstOrLast, HyprlandData, RevIf, WorkspaceData, WorkspaceId,
};
use tracing::{debug, instrument, trace, trace_span, warn};

pub fn find_next_workspace(
    direction: &Direction,
    wrap: bool,
    hypr_data: &HyprlandData,
    active: Active,
    workspaces_per_row: u8,
) -> Active {
    let _span = trace_span!("find_next_workspace", direction = ?direction, wrap = wrap, active = ?active, workspaces_per_row = workspaces_per_row).entered();

    if hypr_data.workspaces.is_empty() {
        debug!("No workspaces available, returning None");
        return active;
    }
    let filtered = hypr_data
        .workspaces
        .iter()
        .filter(|(_, data)| data.any_client_enabled)
        .collect::<Vec<_>>();
    if filtered.len() == 1 {
        trace!("Only one workspaces available, returning current workspaces");
        let workspaces = &filtered[0];
        return Active {
            client: None,
            workspace: workspaces.0,
            monitor: workspaces.1.monitor,
        };
    }
    let current = filtered
        .iter()
        .position(|(id, _)| *id == active.workspace)
        .unwrap_or_else(|| {
            warn!("Active workspace not found in workspaces, returning first workspace");
            0
        });

    let index = find_next_grid(direction, wrap, filtered.len(), current, workspaces_per_row);
    #[allow(clippy::map_unwrap_or)]
    let next_active = filtered
        .get(index)
        .map(|(id, data)| Active {
            client: None,
            workspace: *id,
            monitor: data.monitor,
        })
        .unwrap_or_else(|| {
            warn!("Unable to find next workspace, returning current workspace");
            active
        });
    // .expect("unable to find next workspace!");
    trace!("Next active: {next_active:?}");
    next_active
}

#[instrument(
    level = "trace",
    skip_all,
    ret,
    fields(direction = ?direction, wrap = wrap, active = ?active, clients_per_row = clients_per_row)
)]
pub fn find_next_client(
    direction: &Direction,
    wrap: bool,
    hypr_data: &HyprlandData,
    active: Active,
    clients_per_row: u8,
) -> Active {
    if hypr_data.clients.is_empty() {
        debug!("No clients available, returning None");
        return active;
    }

    #[allow(clippy::option_if_let_else)]
    let next_active = match active.client {
        None => find_first_client(direction, &hypr_data.clients, &hypr_data.workspaces, active),
        Some(client_id) => {
            let filtered = hypr_data
                .clients
                .iter()
                .filter(|(_, data)| data.enabled)
                .collect::<Vec<_>>();
            if filtered.len() == 1 {
                trace!("Only one client available, returning current client");
                let client = &filtered[0];
                return Active {
                    client: Some(client.0),
                    workspace: client.1.workspace,
                    monitor: client.1.monitor,
                };
            }
            let current = filtered
                .iter()
                .position(|(id, _)| *id == client_id)
                .unwrap_or(0);
            let index = find_next_grid(direction, wrap, filtered.len(), current, clients_per_row);
            #[allow(clippy::map_unwrap_or)]
            filtered
                .get(index)
                .map(|(id, data)| Active {
                    client: Some(*id),
                    workspace: data.workspace,
                    monitor: data.monitor,
                })
                .unwrap_or_else(|| {
                    warn!("Unable to find next client, returning current client");
                    active
                })
        }
    };

    trace!("Next active: {next_active:?}");
    next_active
}

fn find_next_grid(
    direction: &Direction,
    wrap: bool,
    filtered_len: usize,
    current: usize,
    items_per_row: u8,
) -> usize {
    let _span = trace_span!("find_next_grid", len = filtered_len, current = current).entered();
    if filtered_len <= 1 {
        return 0;
    }
    let items_per_row = items_per_row as usize;
    let items_per_row = if items_per_row == 0 { 1 } else { items_per_row };

    match direction {
        Direction::Right => {
            let next = current + 1;
            if next >= filtered_len {
                if wrap { 0 } else { current }
            } else {
                next
            }
        }
        Direction::Left => {
            if current == 0 {
                if wrap { filtered_len - 1 } else { current }
            } else {
                current - 1
            }
        }
        Direction::Down => {
            let next = current + items_per_row;
            if next >= filtered_len {
                let current_row = current / items_per_row;
                let total_rows = (filtered_len + items_per_row - 1) / items_per_row;
                if current_row + 1 < total_rows {
                    filtered_len - 1
                } else if wrap {
                    current % items_per_row
                } else {
                    current
                }
            } else {
                next
            }
        }
        Direction::Up => {
            if current < items_per_row {
                if wrap {
                    let total_rows = (filtered_len + items_per_row - 1) / items_per_row;
                    let col = current % items_per_row;
                    let target = (total_rows - 1) * items_per_row + col;
                    if target >= filtered_len {
                        filtered_len - 1
                    } else {
                        target
                    }
                } else {
                    current
                }
            } else {
                current - items_per_row
            }
        }
    }
}

fn find_first_client(
    direction: &Direction,
    clients: &[(ClientId, ClientData)],
    workspaces: &[(WorkspaceId, WorkspaceData)],
    active: Active,
) -> Active {
    let get_last = matches!(direction, Direction::Left | Direction::Up);
    let (id, next) = clients
        .iter()
        .filter(|(_, c)| c.workspace == active.workspace && c.enabled)
        .get_first_or_last(get_last)
        .unwrap_or_else(|| {
            trace!("No client found in current workspace, looking for next client in workspaces");
            workspaces
                .iter()
                .reverse_if(get_last)
                .skip_while(|(id, _)| id != &active.workspace)
                .find_map(|(id, _)| {
                    trace!("Finding first client in workspace {id:?}");
                    clients
                        .iter()
                        .filter(|(_, c)| c.workspace == *id && c.enabled)
                        .get_first_or_last(get_last)
                })
                .unwrap_or_else(|| {
                    clients
                        .iter()
                        .filter(|(_, c)| c.monitor == active.monitor)
                        .get_first_or_last(get_last)
                        .unwrap_or_else(|| {
                            clients
                                .iter()
                                .get_first_or_last(get_last)
                                .expect("clients contain at least two clients")
                        })
                })
        });
    Active {
        client: Some(*id),
        workspace: next.workspace,
        monitor: next.monitor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::type_complexity)]
    fn create_test_data(
        client_count: usize,
        workspace_count: usize,
        enabled: Option<usize>,
    ) -> (
        Vec<(ClientId, ClientData)>,
        Vec<(WorkspaceId, WorkspaceData)>,
    ) {
        let clients: Vec<(ClientId, ClientData)> = (0..client_count)
            .map(|i| {
                (
                    i as ClientId,
                    ClientData {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                        class: String::new(),
                        title: String::new(),
                        #[allow(clippy::cast_possible_wrap)]
                        workspace: (i % workspace_count) as WorkspaceId,
                        monitor: 0,
                        focus_history_id: 0,
                        floating: false,
                        #[allow(clippy::map_unwrap_or)]
                        enabled: enabled.map(|s| s >= i).unwrap_or(true),
                        pid: 0,
                    },
                )
            })
            .collect();

        #[allow(clippy::cast_possible_wrap)]
        let workspaces: Vec<(WorkspaceId, WorkspaceData)> = (0..workspace_count)
            .map(|i| {
                (
                    i as WorkspaceId,
                    WorkspaceData {
                        name: String::new(),
                        width: 0,
                        height: 0,
                        monitor: 0,
                        any_client_enabled: clients
                            .iter()
                            .any(|(_, c)| c.workspace == i as WorkspaceId && c.enabled),
                    },
                )
            })
            .collect();

        (clients, workspaces)
    }

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn test_find_next_grid_scenarios() {
        // Scenario 1: items_per_row = 2, total = 5
        // 0 1
        // 2 3
        // 4
        let len = 5;
        let per_row = 2;

        // Down from 0 -> 2
        assert_eq!(find_next_grid(&Direction::Down, true, len, 0, per_row), 2);
        // Down from 2 -> 4
        assert_eq!(find_next_grid(&Direction::Down, true, len, 2, per_row), 4);
        // Down from 4 -> 0 (wrap)
        assert_eq!(find_next_grid(&Direction::Down, true, len, 4, per_row), 0);

        // Scenario 2: Down from 3 -> 4
        assert_eq!(find_next_grid(&Direction::Down, true, len, 3, per_row), 4);

        // Up from 0 -> 4 (wrap)
        assert_eq!(find_next_grid(&Direction::Up, true, len, 0, per_row), 4);
        // Up from 1 -> 4 (wrap, goes to only item in last row)
        assert_eq!(find_next_grid(&Direction::Up, true, len, 1, per_row), 4);
        
        // Scenario 3: Right wrap
        assert_eq!(find_next_grid(&Direction::Right, true, len, 4, per_row), 0);
        // Left wrap
        assert_eq!(find_next_grid(&Direction::Left, true, len, 0, per_row), 4);
    }

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn test_find_next_workspace_0() {
        let (clients, workspaces) = create_test_data(0, 0, None);
        let hypr_data = HyprlandData {
            clients,
            workspaces,
            monitors: vec![],
        };
        let active = Active {
            client: None,
            workspace: 0,
            monitor: 0,
        };
        trace!("data: {hypr_data:?}");

        assert_eq!(
            find_next_workspace(&Direction::Right, true, &hypr_data, active, 3),
            active
        );
    }
}
