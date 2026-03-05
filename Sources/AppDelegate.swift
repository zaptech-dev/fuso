import AppKit
import SwiftUI

class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusItem: NSStatusItem!
    private var popover: NSPopover!

    func applicationDidFinishLaunching(_ notification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)

        if let button = statusItem.button {
            button.image = createMenuBarIcon()
            button.action = #selector(togglePopover)
            button.target = self
        }

        popover = NSPopover()
        popover.behavior = .transient
        popover.appearance = NSAppearance(named: .aqua)
        popover.contentViewController = NSHostingController(
            rootView: ClockListView(onQuit: { NSApp.terminate(nil) })
                .preferredColorScheme(.light)
        )
    }

    private func createMenuBarIcon() -> NSImage {
        let size = NSSize(width: 18, height: 18)
        let image = NSImage(size: size, flipped: false) { rect -> Bool in
            let inset: CGFloat = 1.0
            let circleRect = rect.insetBy(dx: inset, dy: inset)
            let center = NSPoint(x: rect.midX, y: rect.midY)
            let radius = circleRect.width / 2

            NSColor.black.setStroke()
            NSColor.black.setFill()

            // Clock outline
            let circle = NSBezierPath(ovalIn: circleRect)
            circle.lineWidth = 1.5
            circle.stroke()

            // Hour ticks — small marks around the edge
            for i in 0..<12 {
                let angle = CGFloat(i) * 30.0 * .pi / 180.0
                let outerR = radius - 1.0
                let innerR = (i % 3 == 0) ? radius - 3.5 : radius - 2.5
                let outer = NSPoint(
                    x: center.x + outerR * sin(angle),
                    y: center.y + outerR * cos(angle)
                )
                let inner = NSPoint(
                    x: center.x + innerR * sin(angle),
                    y: center.y + innerR * cos(angle)
                )
                let tick = NSBezierPath()
                tick.move(to: inner)
                tick.line(to: outer)
                tick.lineWidth = (i % 3 == 0) ? 1.3 : 0.8
                tick.stroke()
            }

            // Hour hand (short, pointing ~10 o'clock)
            let hourAngle: CGFloat = 300 * .pi / 180
            let hourLen = radius * 0.45
            let hourEnd = NSPoint(
                x: center.x + hourLen * sin(hourAngle),
                y: center.y + hourLen * cos(hourAngle)
            )
            let hourHand = NSBezierPath()
            hourHand.move(to: center)
            hourHand.line(to: hourEnd)
            hourHand.lineWidth = 1.8
            hourHand.lineCapStyle = .round
            hourHand.stroke()

            // Minute hand (long, pointing ~2 o'clock)
            let minAngle: CGFloat = 60 * .pi / 180
            let minLen = radius * 0.7
            let minEnd = NSPoint(
                x: center.x + minLen * sin(minAngle),
                y: center.y + minLen * cos(minAngle)
            )
            let minHand = NSBezierPath()
            minHand.move(to: center)
            minHand.line(to: minEnd)
            minHand.lineWidth = 1.2
            minHand.lineCapStyle = .round
            minHand.stroke()

            // Center dot
            let dotSize: CGFloat = 2.2
            let dot = NSBezierPath(ovalIn: NSRect(
                x: center.x - dotSize / 2,
                y: center.y - dotSize / 2,
                width: dotSize, height: dotSize
            ))
            dot.fill()

            return true
        }
        image.isTemplate = true
        return image
    }

    @objc private func togglePopover() {
        if popover.isShown {
            popover.performClose(nil)
        } else if let button = statusItem.button {
            popover.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)
            NSApp.activate(ignoringOtherApps: true)
        }
    }
}
