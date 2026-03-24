#include "globals.h"

void exit() {
    if constexpr (SWITCHAROO_PRINT_DEBUG == 1) {
        HyprlandAPI::addNotification(PHANDLE, "[Switcharoo Plugin] Plugin deactivated", RED, 3000);
    }
}
