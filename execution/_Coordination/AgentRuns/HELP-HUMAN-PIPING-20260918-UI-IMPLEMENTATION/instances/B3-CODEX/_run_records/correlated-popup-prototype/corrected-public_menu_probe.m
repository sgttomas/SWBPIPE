#import <AppKit/AppKit.h>
#include <stdio.h>
typedef struct { uint64_t generation; uint64_t windowEpoch; int64_t tracking; int64_t action; int64_t endKey; } PopupSnapshot;
static PopupSnapshot popup = {0, 0, 0, 0, -1};
static __weak NSMenu *trackedMenu;
PopupSnapshot public_menu_snapshot(void) {
    NSCAssert([NSThread isMainThread], @"snapshot must run on main thread");
    fprintf(stderr, "PROBE_SNAPSHOT generation=%llu epoch=%llu tracking=%lld action=%lld endKey=%lld\n", popup.generation, popup.windowEpoch, popup.tracking, popup.action, popup.endKey);
    return popup;
}
void public_menu_probe_install(void *owningWindowPointer) {
    NSWindow *owningWindow = (__bridge NSWindow *)owningWindowPointer;
    static NSMutableArray *tokens;
    if (tokens) return;
    tokens = [[NSMutableArray alloc] init];
    for (NSNotificationName name in @[NSMenuDidBeginTrackingNotification, NSMenuDidEndTrackingNotification, NSMenuWillSendActionNotification, NSMenuDidSendActionNotification]) {
        id token = [[NSNotificationCenter defaultCenter] addObserverForName:name object:nil queue:nil usingBlock:^(NSNotification *note) {
            NSEvent *event = NSApp.currentEvent;
            NSInteger type = event ? event.type : -1;
            NSInteger controlCode = -1;
            if (event && (type == NSEventTypeKeyDown || type == NSEventTypeKeyUp || type == NSEventTypeFlagsChanged)) {
                unsigned short code = event.keyCode;
                if (code == 36 || code == 48 || code == 49 || code == 53 || code == 76 || (code >= 123 && code <= 126)) controlCode = code;
            }
            if ([note.name isEqualToString:NSMenuDidBeginTrackingNotification]) {
                popup.generation++; popup.tracking = 1; popup.action = 0; popup.endKey = -1; trackedMenu = note.object;
            } else if (note.object == trackedMenu) {
                if ([note.name isEqualToString:NSMenuDidEndTrackingNotification]) { popup.tracking = 0; popup.endKey = controlCode; }
                if ([note.name isEqualToString:NSMenuDidSendActionNotification]) popup.action = 1;
            }
            fprintf(stderr, "PROBE_MENU time=%.6f notification=%s menu=%p eventType=%ld controlKeyCode=%ld\n", [NSDate timeIntervalSinceReferenceDate], note.name.UTF8String, (__bridge void *)note.object, (long)type, (long)controlCode);
            fflush(stderr);
        }];
        [tokens addObject:token];
    }
    for (NSNotificationName name in @[NSWindowDidResignKeyNotification, NSWindowDidBecomeKeyNotification]) {
        [tokens addObject:[[NSNotificationCenter defaultCenter] addObserverForName:name object:nil queue:nil usingBlock:^(NSNotification *note) {
            BOOL isOwner = note.object == owningWindow;
            if (isOwner && [note.name isEqualToString:NSWindowDidResignKeyNotification]) popup.windowEpoch++;
            fprintf(stderr, "PROBE_WINDOW notification=%s owningWebview=%d windowNumber=%ld epoch=%llu\n", note.name.UTF8String, isOwner, (long)[note.object windowNumber], popup.windowEpoch);
        }]];
    }
    fprintf(stderr, "PROBE_MENU installed public observers\n");
}
