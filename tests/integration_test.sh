#!/bin/bash
# Integration test script for trello-rs CLI
# Creates a test board, exercises all commands, and cleans up.
set -euo pipefail

BIN="./target/debug/trello-rs"
PASS=0
FAIL=0
TEST_BOARD_ID=""
TEST_LIST_ID=""
TEST_CARD_ID=""
TEST_LABEL_ID=""
TEST_CHECKLIST_ID=""

pass() { echo "  PASS $1"; PASS=$((PASS+1)); }
fail() { echo "  FAIL $1: $2"; FAIL=$((FAIL+1)); }
skip() { echo "  SKIP $1 ($2)"; }

cleanup() {
    echo ""
    echo "=== Cleanup ==="
    if [ -n "${TEST_CHECKLIST_ID}" ]; then
        echo "Deleting checklist $TEST_CHECKLIST_ID..."
        $BIN checklist delete "$TEST_CHECKLIST_ID" 2>/dev/null && echo "  Checklist deleted" || echo "  (cleanup: checklist delete failed)"
    fi
    if [ -n "${TEST_LABEL_ID}" ]; then
        echo "Deleting label $TEST_LABEL_ID..."
        $BIN label delete "$TEST_LABEL_ID" 2>/dev/null && echo "  Label deleted" || echo "  (cleanup: label delete failed)"
    fi
    if [ -n "${TEST_CARD_ID}" ]; then
        echo "Deleting card $TEST_CARD_ID..."
        $BIN card delete "$TEST_CARD_ID" 2>/dev/null && echo "  Card deleted" || echo "  (cleanup: card delete failed)"
    fi
    if [ -n "${TEST_LIST_ID}" ]; then
        echo "Closing list $TEST_LIST_ID..."
        $BIN list close "$TEST_LIST_ID" 2>/dev/null && echo "  List closed" || echo "  (cleanup: list close failed)"
    fi
    if [ -n "${TEST_BOARD_ID}" ]; then
        echo "Deleting board $TEST_BOARD_ID..."
        $BIN board delete "$TEST_BOARD_ID" 2>/dev/null && echo "  Board deleted" || echo "  (cleanup: board delete failed)"
    fi
}
trap cleanup EXIT

# ── Build ──
echo "=== Building ==="
cargo build 2>&1 || { echo "BUILD FAILED"; exit 1; }
echo ""

# ═══════════════════════════════════════════════════════════════════
# BOARD
# ═══════════════════════════════════════════════════════════════════
echo "=== Board ==="

echo "--- board list ---"
if OUTPUT=$($BIN board list 2>&1); then
    pass "board list"
    EXISTING_BOARD_ID=$(echo "$OUTPUT" | jq -r '.[0].id // empty')
    echo "       Found $(echo "$OUTPUT" | jq '. | length') boards"
else
    fail "board list" "$OUTPUT"
fi

echo "--- board list --filter open ---"
if OUTPUT=$($BIN board list --filter open 2>&1); then
    pass "board list --filter"
else
    fail "board list --filter" "$OUTPUT"
fi

echo "--- board create ---"
TEST_BOARD_NAME="trello-rs-test-$(date +%s)"
if OUTPUT=$($BIN board create --name "$TEST_BOARD_NAME" --desc "integration test board" 2>&1); then
    TEST_BOARD_ID=$(echo "$OUTPUT" | jq -r '.id // empty')
    if [ -n "$TEST_BOARD_ID" ] && [ "$TEST_BOARD_ID" != "null" ]; then
        pass "board create"
        echo "       Created: $TEST_BOARD_ID"
    else
        fail "board create" "no id in response: $OUTPUT"
    fi
else
    fail "board create" "$OUTPUT"
fi

echo "--- board get ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN board get "$TEST_BOARD_ID" 2>&1); then
        GOT_NAME=$(echo "$OUTPUT" | jq -r '.name // empty')
        if [ "$GOT_NAME" = "$TEST_BOARD_NAME" ]; then
            pass "board get"
        else
            fail "board get" "expected name '$TEST_BOARD_NAME', got '$GOT_NAME'"
        fi
    else
        fail "board get" "$OUTPUT"
    fi
fi

echo "--- board get (existing) ---"
if [ -n "${EXISTING_BOARD_ID:-}" ]; then
    if OUTPUT=$($BIN board get "$EXISTING_BOARD_ID" 2>&1); then
        pass "board get (existing)"
    else
        fail "board get (existing)" "$OUTPUT"
    fi
else
    skip "board get (existing)" "no existing boards"
fi

echo "--- board update ---"
if [ -n "$TEST_BOARD_ID" ]; then
    UPDATED_NAME="${TEST_BOARD_NAME}-updated"
    if OUTPUT=$($BIN board update "$TEST_BOARD_ID" --name "$UPDATED_NAME" 2>&1); then
        GOT_NAME=$(echo "$OUTPUT" | jq -r '.name // empty')
        if [ "$GOT_NAME" = "$UPDATED_NAME" ]; then
            pass "board update"
        else
            fail "board update" "expected '$UPDATED_NAME', got '$GOT_NAME'"
        fi
    else
        fail "board update" "$OUTPUT"
    fi
fi

# ═══════════════════════════════════════════════════════════════════
# LIST
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== List ==="

echo "--- list create ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN list create --name "Test List" --board-id "$TEST_BOARD_ID" 2>&1); then
        TEST_LIST_ID=$(echo "$OUTPUT" | jq -r '.id // empty')
        if [ -n "$TEST_LIST_ID" ] && [ "$TEST_LIST_ID" != "null" ]; then
            pass "list create"
            echo "       Created: $TEST_LIST_ID"
        else
            fail "list create" "no id in response"
        fi
    else
        fail "list create" "$OUTPUT"
    fi
fi

echo "--- list list ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN list list --board-id "$TEST_BOARD_ID" 2>&1); then
        pass "list list"
    else
        fail "list list" "$OUTPUT"
    fi
fi

echo "--- list get ---"
if [ -n "$TEST_LIST_ID" ]; then
    if OUTPUT=$($BIN list get "$TEST_LIST_ID" 2>&1); then
        pass "list get"
    else
        fail "list get" "$OUTPUT"
    fi
fi

echo "--- list update ---"
if [ -n "$TEST_LIST_ID" ]; then
    if OUTPUT=$($BIN list update "$TEST_LIST_ID" --name "Test List Updated" 2>&1); then
        pass "list update"
    else
        fail "list update" "$OUTPUT"
    fi
fi

# ═══════════════════════════════════════════════════════════════════
# CARD
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Card ==="

echo "--- card create ---"
if [ -n "$TEST_LIST_ID" ]; then
    if OUTPUT=$($BIN card create --name "Test Card" --list-id "$TEST_LIST_ID" --desc "integration test card" 2>&1); then
        TEST_CARD_ID=$(echo "$OUTPUT" | jq -r '.id // empty')
        if [ -n "$TEST_CARD_ID" ] && [ "$TEST_CARD_ID" != "null" ]; then
            pass "card create"
            echo "       Created: $TEST_CARD_ID"
        else
            fail "card create" "no id in response"
        fi
    else
        fail "card create" "$OUTPUT"
    fi
fi

echo "--- card get ---"
if [ -n "$TEST_CARD_ID" ]; then
    if OUTPUT=$($BIN card get "$TEST_CARD_ID" 2>&1); then
        pass "card get"
    else
        fail "card get" "$OUTPUT"
    fi
fi

echo "--- card list --list-id ---"
if [ -n "$TEST_LIST_ID" ]; then
    if OUTPUT=$($BIN card list --list-id "$TEST_LIST_ID" 2>&1); then
        CARD_COUNT=$(echo "$OUTPUT" | jq '. | length' 2>/dev/null)
        if [ "${CARD_COUNT:-0}" -gt 0 ]; then
            pass "card list --list-id"
            echo "       Found $CARD_COUNT card(s) in list"
        else
            fail "card list --list-id" "no cards found in list"
        fi
    else
        fail "card list --list-id" "$OUTPUT"
    fi
fi

echo "--- card list --board-id ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN card list --board-id "$TEST_BOARD_ID" 2>&1); then
        pass "card list --board-id"
    else
        fail "card list --board-id" "$OUTPUT"
    fi
fi

echo "--- card update ---"
if [ -n "$TEST_CARD_ID" ]; then
    if OUTPUT=$($BIN card update "$TEST_CARD_ID" --name "Test Card Updated" --desc "updated" 2>&1); then
        pass "card update"
    else
        fail "card update" "$OUTPUT"
    fi
fi

# ═══════════════════════════════════════════════════════════════════
# LABEL
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Label ==="

echo "--- label create ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN label create --name "test-label" --color red --board-id "$TEST_BOARD_ID" 2>&1); then
        TEST_LABEL_ID=$(echo "$OUTPUT" | jq -r '.id // empty')
        if [ -n "$TEST_LABEL_ID" ] && [ "$TEST_LABEL_ID" != "null" ]; then
            pass "label create"
            echo "       Created: $TEST_LABEL_ID"
        else
            fail "label create" "no id in response"
        fi
    else
        fail "label create" "$OUTPUT"
    fi
fi

echo "--- label list ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN label list --board-id "$TEST_BOARD_ID" 2>&1); then
        pass "label list"
    else
        fail "label list" "$OUTPUT"
    fi
fi

echo "--- label get ---"
if [ -n "$TEST_LABEL_ID" ]; then
    if OUTPUT=$($BIN label get "$TEST_LABEL_ID" 2>&1); then
        pass "label get"
    else
        fail "label get" "$OUTPUT"
    fi
fi

echo "--- label update ---"
if [ -n "$TEST_LABEL_ID" ]; then
    if OUTPUT=$($BIN label update "$TEST_LABEL_ID" --name "updated-label" --color blue 2>&1); then
        pass "label update"
    else
        fail "label update" "$OUTPUT"
    fi
fi

echo "--- label delete ---"
if [ -n "$TEST_LABEL_ID" ]; then
    if OUTPUT=$($BIN label delete "$TEST_LABEL_ID" 2>&1); then
        pass "label delete"
    else
        fail "label delete" "$OUTPUT"
    fi
    TEST_LABEL_ID=""
fi

# ═══════════════════════════════════════════════════════════════════
# CHECKLIST
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Checklist ==="

echo "--- checklist create ---"
if [ -n "$TEST_CARD_ID" ]; then
    if OUTPUT=$($BIN checklist create --name "Test Checklist" --card-id "$TEST_CARD_ID" 2>&1); then
        TEST_CHECKLIST_ID=$(echo "$OUTPUT" | jq -r '.id // empty')
        if [ -n "$TEST_CHECKLIST_ID" ] && [ "$TEST_CHECKLIST_ID" != "null" ]; then
            pass "checklist create"
            echo "       Created: $TEST_CHECKLIST_ID"
        else
            fail "checklist create" "no id in response"
        fi
    else
        fail "checklist create" "$OUTPUT"
    fi
fi

echo "--- checklist get ---"
if [ -n "$TEST_CHECKLIST_ID" ]; then
    if OUTPUT=$($BIN checklist get "$TEST_CHECKLIST_ID" 2>&1); then
        pass "checklist get"
    else
        fail "checklist get" "$OUTPUT"
    fi
fi

echo "--- checklist update ---"
if [ -n "$TEST_CHECKLIST_ID" ]; then
    if OUTPUT=$($BIN checklist update "$TEST_CHECKLIST_ID" --name "Test Checklist Updated" 2>&1); then
        pass "checklist update"
    else
        fail "checklist update" "$OUTPUT"
    fi
fi

# ═══════════════════════════════════════════════════════════════════
# MEMBER
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Member ==="

echo "--- member get (default) ---"
if OUTPUT=$($BIN member get 2>&1); then
    pass "member get"
    MEMBER_USERNAME=$(echo "$OUTPUT" | jq -r '.username // empty')
    MEMBER_ID=$(echo "$OUTPUT" | jq -r '.id // empty')
    echo "       Username: $MEMBER_USERNAME"
else
    fail "member get" "$OUTPUT"
fi

echo "--- member get me ---"
if OUTPUT=$($BIN member get me 2>&1); then
    pass "member get me"
else
    fail "member get me" "$OUTPUT"
fi

echo "--- member boards ---"
if OUTPUT=$($BIN member boards 2>&1); then
    pass "member boards"
else
    fail "member boards" "$OUTPUT"
fi

echo "--- member boards --filter open ---"
if OUTPUT=$($BIN member boards --filter open 2>&1); then
    pass "member boards --filter"
else
    fail "member boards --filter" "$OUTPUT"
fi

echo "--- member update ---"
if [ -n "${MEMBER_ID:-}" ]; then
    if OUTPUT=$($BIN member update "$MEMBER_ID" --bio "test $(date +%s)" 2>&1); then
        pass "member update"
    elif echo "$OUTPUT" | grep -q "authentication failed"; then
        skip "member update" "token lacks write permission for member profile"
    else
        fail "member update" "$OUTPUT"
    fi
fi

# ═══════════════════════════════════════════════════════════════════
# ACTION
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Action ==="

echo "--- action board ---"
if [ -n "${EXISTING_BOARD_ID:-}" ]; then
    if OUTPUT=$($BIN action board --board-id "$EXISTING_BOARD_ID" 2>&1); then
        pass "action board"
        ACTION_ID=$(echo "$OUTPUT" | jq -r '.[0].id // empty')
    else
        fail "action board" "$OUTPUT"
    fi
else
    skip "action board" "no existing boards"
fi

echo "--- action board --filter ---"
if [ -n "${EXISTING_BOARD_ID:-}" ]; then
    if OUTPUT=$($BIN action board --board-id "$EXISTING_BOARD_ID" --filter createCard 2>&1); then
        pass "action board --filter"
    else
        fail "action board --filter" "$OUTPUT"
    fi
else
    skip "action board --filter" "no existing boards"
fi

echo "--- action card ---"
if [ -n "$TEST_CARD_ID" ]; then
    if OUTPUT=$($BIN action card --card-id "$TEST_CARD_ID" 2>&1); then
        pass "action card"
    else
        fail "action card" "$OUTPUT"
    fi
fi

echo "--- action get ---"
if [ -n "${ACTION_ID:-}" ]; then
    if OUTPUT=$($BIN action get "$ACTION_ID" 2>&1); then
        pass "action get"
    else
        fail "action get" "$OUTPUT"
    fi
else
    skip "action get" "no action id available"
fi

# ═══════════════════════════════════════════════════════════════════
# SEARCH
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Search ==="

echo "--- search ---"
if OUTPUT=$($BIN search "board" 2>&1); then
    pass "search"
else
    fail "search" "$OUTPUT"
fi

echo "--- search --members ---"
if OUTPUT=$($BIN search "a" --members 2>&1); then
    pass "search --members"
else
    fail "search --members" "$OUTPUT"
fi

echo "--- search --model-types ---"
if OUTPUT=$($BIN search "a" --model-types boards,cards --limit 3 2>&1); then
    pass "search with options"
else
    fail "search with options" "$OUTPUT"
fi

# ═══════════════════════════════════════════════════════════════════
# EMOJI
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Emoji ==="

echo "--- emoji ---"
if OUTPUT=$($BIN emoji 2>&1); then
    pass "emoji"
    EMOJI_COUNT=$(echo "$OUTPUT" | jq '. | length' 2>/dev/null || echo "?")
    echo "       Available: $EMOJI_COUNT emoji"
else
    fail "emoji" "$OUTPUT"
fi

# ═══════════════════════════════════════════════════════════════════
# TOKEN
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Token ==="

echo "--- token get ---"
if OUTPUT=$($BIN token get 2>&1); then
    pass "token get"
else
    fail "token get" "$OUTPUT"
fi

# ═══════════════════════════════════════════════════════════════════
# WEBHOOK
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Webhook ==="

echo "--- webhook list ---"
if OUTPUT=$($BIN webhook list 2>&1); then
    pass "webhook list"
    WEBHOOK_ID=$(echo "$OUTPUT" | jq -r '.[0].id // empty')
else
    fail "webhook list" "$OUTPUT"
fi

echo "--- webhook get ---"
if [ -n "${WEBHOOK_ID:-}" ] && [ "${WEBHOOK_ID:-}" != "null" ]; then
    if OUTPUT=$($BIN webhook get "$WEBHOOK_ID" 2>&1); then
        pass "webhook get"
    else
        fail "webhook get" "$OUTPUT"
    fi
else
    skip "webhook get" "no existing webhooks"
fi

# ═══════════════════════════════════════════════════════════════════
# BATCH
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Batch ==="

echo "--- batch ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN batch "/boards/$TEST_BOARD_ID" 2>&1); then
        pass "batch (single url)"
    else
        fail "batch (single url)" "$OUTPUT"
    fi
fi

echo "--- batch (multiple) ---"
if [ -n "$TEST_BOARD_ID" ] && [ -n "${EXISTING_BOARD_ID:-}" ]; then
    if OUTPUT=$($BIN batch "/boards/$TEST_BOARD_ID,/boards/$EXISTING_BOARD_ID" 2>&1); then
        pass "batch (multiple urls)"
    else
        fail "batch (multiple urls)" "$OUTPUT"
    fi
fi

# ═══════════════════════════════════════════════════════════════════
# ORGANIZATION (read-only, if org exists)
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Organization ==="

ORG_ID=$($BIN member get 2>/dev/null | jq -r '.idOrganizations[0] // empty')
echo "--- organization get ---"
if [ -n "$ORG_ID" ] && [ "$ORG_ID" != "null" ]; then
    if OUTPUT=$($BIN organization get "$ORG_ID" 2>&1); then
        pass "organization get"
    else
        fail "organization get" "$OUTPUT"
    fi
else
    skip "organization get" "user has no organizations"
fi

# ═══════════════════════════════════════════════════════════════════
# CLEANUP (also tests delete/close commands)
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Cleanup ==="

echo "--- checklist delete ---"
if [ -n "$TEST_CHECKLIST_ID" ]; then
    if OUTPUT=$($BIN checklist delete "$TEST_CHECKLIST_ID" 2>&1); then
        pass "checklist delete"
    else
        fail "checklist delete" "$OUTPUT"
    fi
    TEST_CHECKLIST_ID=""
fi

echo "--- card delete ---"
if [ -n "$TEST_CARD_ID" ]; then
    if OUTPUT=$($BIN card delete "$TEST_CARD_ID" 2>&1); then
        pass "card delete"
    else
        fail "card delete" "$OUTPUT"
    fi
    TEST_CARD_ID=""
fi

echo "--- list close ---"
if [ -n "$TEST_LIST_ID" ]; then
    if OUTPUT=$($BIN list close "$TEST_LIST_ID" 2>&1); then
        pass "list close"
    else
        fail "list close" "$OUTPUT"
    fi
    TEST_LIST_ID=""
fi

echo "--- board delete ---"
if [ -n "$TEST_BOARD_ID" ]; then
    if OUTPUT=$($BIN board delete "$TEST_BOARD_ID" 2>&1); then
        pass "board delete"
    else
        fail "board delete" "$OUTPUT"
    fi
    TEST_BOARD_ID=""
fi

# ═══════════════════════════════════════════════════════════════════
# SKIPPED COMMANDS
# ═══════════════════════════════════════════════════════════════════
echo ""
echo "=== Skipped ==="
skip "notification get"       "requires notification ID"
skip "custom-field get"       "requires custom field ID"
skip "enterprise get"         "requires enterprise ID"
skip "plugin get"             "requires plugin ID"
skip "webhook create"         "requires callback URL reachable by Trello"
skip "webhook update"         "requires webhook ID"
skip "webhook delete"         "requires webhook ID"
skip "organization create"    "requires team/enterprise permissions"
skip "organization update"    "requires team/enterprise permissions"
skip "organization delete"    "requires team/enterprise permissions"

# ═══════════════════════════════════════════════════════════════════
echo ""
echo "========================================="
printf "Results: %d passed, %d failed\n" "$PASS" "$FAIL"
echo "========================================="

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
