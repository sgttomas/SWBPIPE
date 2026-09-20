#import <AppKit/AppKit.h>
#include <stdio.h>
void public_menu_probe_install(void) {
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
            fprintf(stderr, "PROBE_MENU time=%.6f notification=%s menu=%p eventType=%ld controlKeyCode=%ld\n", [NSDate timeIntervalSinceReferenceDate], note.name.UTF8String, (__bridge void *)note.object, (long)type, (long)controlCode);
            fflush(stderr);
        }];
        [tokens addObject:token];
    }
    fprintf(stderr, "PROBE_MENU installed public observers\n");
}
