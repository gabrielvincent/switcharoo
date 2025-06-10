use core_lib::transfer::Direction;
use core_lib::{
    Active, ClientData, ClientId, GetFirstOrLast, HyprlandData, WorkspaceData, WorkspaceId,
};
use tracing::{Level, debug, span};

pub fn find_next(
    direction: &Direction,
    workspace: bool,
    hypr_data: &HyprlandData,
    active: Active,
    workspaces_per_row: usize,
) -> Active {
    let _span =
        span!(Level::TRACE, "find_next", direction = ?direction, workspace, active = ?active)
            .entered();
    if workspace {
        find_next_workspace(
            direction,
            &hypr_data.workspaces,
            active.workspace,
            workspaces_per_row,
        )
    } else {
        if let Some(active_client) = active.client {
            find_next_client(
                direction,
                &hypr_data.clients,
                active_client,
                workspaces_per_row,
            )
        } else {
            find_first_client(direction, &hypr_data.clients, active)
        }
    }
    .unwrap_or(active)
}

pub fn find_first_client(
    direction: &Direction,
    clients: &[(ClientId, ClientData)],
    active: Active,
) -> Option<Active> {
    let get_last = matches!(direction, Direction::Left | Direction::Up);
    clients
        .iter()
        .filter(|(_, c)| c.workspace == active.workspace)
        .get_first_or_last(get_last)
        .or_else(|| {
            clients
                .iter()
                .filter(|(_, c)| c.monitor == active.monitor)
                .get_first_or_last(get_last)
                .or_else(|| clients.iter().get_first_or_last(get_last))
        })
        .map(|(id, data)| Active {
            client: Some(*id),
            workspace: data.workspace,
            monitor: data.monitor,
        })
}

pub fn find_next_client(
    direction: &Direction,
    clients: &[(ClientId, ClientData)],
    active: ClientId,
    clients_per_row: usize,
) -> Option<Active> {
    if clients.is_empty() {
        debug!("No clients available, returning None");
        return None;
    }

    let offset = match direction {
        Direction::Right => 1,
        Direction::Left => -1,
        Direction::Up => -(clients_per_row as isize),
        Direction::Down => clients_per_row as isize,
    };

    clients
        .iter()
        .skip_while(|(id, _)| *id == active) // skip until current element
        .skip(1)
        .filter(|(_, c)| c.enabled) // skip the current element itself
        .cycle()
        .nth(offset.rem_euclid(clients.len() as isize) as usize)
        .map(|(id, data)| Active {
            client: Some(*id),
            workspace: data.workspace,
            monitor: data.monitor,
        })
}

pub fn find_next_workspace(
    direction: &Direction,
    workspaces: &[(WorkspaceId, WorkspaceData)],
    active: WorkspaceId,
    workspaces_per_row: usize,
) -> Option<Active> {
    if workspaces.is_empty() {
        debug!("No workspaces available, returning None");
        return None;
    }

    let offset = match direction {
        Direction::Right => 1,
        Direction::Left => -1,
        Direction::Up => -(workspaces_per_row as isize),
        Direction::Down => workspaces_per_row as isize,
    };

    workspaces
        .into_iter()
        .skip_while(|(id, _)| *id == active) // skip until current element
        .skip(1) // skip the current element itself
        .filter(|(_, c)| c.enabled)
        .cycle()
        .nth(offset.rem_euclid(workspaces.len() as isize) as usize)
        .map(|(id, data)| Active {
            client: None,
            workspace: *id,
            monitor: data.monitor,
        })
}
