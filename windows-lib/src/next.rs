use core_lib::transfer::Direction;
use core_lib::{
    Active, ClientData, ClientId, GetFirstOrLast, HyprlandData, RevIf, WorkspaceData, WorkspaceId,
};
use tracing::{Level, debug, span, trace};

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
    let next = if workspace {
        find_next_workspace(
            direction,
            &hypr_data.workspaces,
            active.workspace,
            workspaces_per_row,
        )
    } else if let Some(active_client) = active.client {
        find_next_client(
            direction,
            &hypr_data.clients,
            active_client,
            workspaces_per_row,
        )
    } else {
        find_first_client(direction, &hypr_data.clients, &hypr_data.workspaces, active)
    }
    .unwrap_or(active);
    trace!("Next active: {:?}", next);
    next
}

pub fn find_first_client(
    direction: &Direction,
    clients: &[(ClientId, ClientData)],
    workspaces: &[(WorkspaceId, WorkspaceData)],
    active: Active,
) -> Option<Active> {
    let get_last = matches!(direction, Direction::Left | Direction::Up);
    clients
        .iter()
        .filter(|(_, c)| c.workspace == active.workspace)
        .get_first_or_last(get_last)
        .or_else(|| {
            trace!("No client found in current workspace, looking for next client in workspaces");
            workspaces
                .iter()
                .rev_if(get_last)
                .skip_while(|(id, _)| id != &active.workspace)
                .find_map(|(id, _)| {
                    trace!("Finding first client in workspace {id:?}");
                    clients
                        .iter()
                        .filter(|(_, c)| c.workspace == *id && c.enabled)
                        .get_first_or_last(get_last)
                })
                .or_else(|| {
                    clients
                        .iter()
                        .filter(|(_, c)| c.monitor == active.monitor)
                        .get_first_or_last(get_last)
                        .or_else(|| clients.iter().get_first_or_last(get_last))
                })
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

    // TODO still buggy when clients get filtered

    let offset = match direction {
        Direction::Right => 1,
        Direction::Left => -1,
        Direction::Up => -(clients_per_row as isize),
        Direction::Down => clients_per_row as isize,
    }
    .rem_euclid(clients.len() as isize) as usize;

    clients
        .iter()
        .cycle()
        .skip_while(|(id, _)| *id != active) // skip until current element
        .filter(|(_, c)| c.enabled)
        .nth(offset)
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
    }
    .rem_euclid(workspaces.len() as isize) as usize;
    trace!("Finding next workspace with offset: {}", offset);

    workspaces
        .iter()
        .cycle()
        .skip_while(|(id, _)| *id != active) // skip until current element
        // .inspect(|w| trace!("Inspecting workspace {w:?}"))
        .nth(offset)
        .map(|(id, data)| Active {
            client: None,
            workspace: *id,
            monitor: data.monitor,
        })
}
