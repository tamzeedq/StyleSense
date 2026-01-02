# StyleSense - Test Plan

A streamlined testing approach for the StyleSense VS Code extension.

## Prerequisites

1. Extension built: `npm run compile`
2. LSP server built: `cd lsp && cargo build`
3. Test file available: `test-code/test-file.cpp`

## Test 1: Extension Activation

**Objective**: Verify extension activation with C/C++ files

**Steps**:
1. Launch VS Code with the extension (`F5` in debug mode)
2. Open `test-code/test-file.cpp`
3. Check Output panel (select "StyleSense Language Server")

**Expected**: Extension activates and server initialization message appears

## Test 2: Style Violation Detection

**Objective**: Verify style issue detection

**Steps**:
1. Open `test-code/test-file.cpp`
2. Wait a moment for LSP analysis
3. Look for yellow squiggles on lines with intentional style issues

**Expected**: Yellow squiggles appear where spacing issues exist (e.g., `x=5`, `if(x>3)`)

## Test 3: Command Functionality

**Objective**: Test the extension command

**Steps**:
1. Open Command Palette (Ctrl+Shift+P)
2. Type and select "StyleSense: Hello World"

**Expected**: Notification appears saying "Hello from StyleSense!"

## Test 4: Settings Application

**Objective**: Test extension settings

**Steps**:
1. Add to settings.json:
   ```json
   "stylesense.enabled": true,
   "stylesense.trace.server": "verbose"
   ```
2. Reload window
3. Open a C++ file

**Expected**: More detailed logs appear in the output channel

## Test 5: Automated Tests

**Objective**: Verify test infrastructure works

**Steps**:
1. Run `npm run test` in terminal

**Expected**: All tests pass without errors

## Troubleshooting

If you encounter issues:

1. Check LSP server path in `extension.ts`
2. Ensure Rust and TypeScript builds succeeded
3. Check Output panel for error messages
4. Verify file permissions for the LSP executable

## Regression Test Checklist

- [ ] Extension activates with C/C++ files
- [ ] Style violations appear as yellow squiggles
- [ ] "Hello World" command works
- [ ] Settings properly apply
- [ ] No console errors in Developer Tools
