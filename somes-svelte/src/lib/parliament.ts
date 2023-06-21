import type { Delegate } from "./types";

export interface Bubble {
    r: number,
    x: number;
    y: number;
    del: Delegate | null;
    color: string | null;
    opacity: number;
}


export function generateHalfCircle(n: number, r: number, w: number, h: number) {
    let smaller_n = n - 2;

    let scaled_max_angle = 18000.0;
    let modulo = scaled_max_angle / smaller_n;
    let count_to = scaled_max_angle + modulo + 1;

    let normalize = 100. * count_to / scaled_max_angle;
    let circles: { x: number, y: number }[] = [];

    for (let angle_deg = count_to; angle_deg > 0; angle_deg -= modulo) {
        let angle_rad = -(angle_deg / normalize) * Math.PI / 180.;
        
        const x = 2.0 * r * Math.cos(angle_rad) + w / 2;
        const y = 2.0 * r * Math.sin(angle_rad) + h / 2;

        const circle = {
            x, y
        };
        circles = circles.concat(circle);
        // circles.push(circle);
    }
    return circles
}


export function partyToColor(party: string): string { 
    switch (party) {
        case "SPÖ":
            return "#E31E2D";
        case "ÖVP":
            return "#62C3D0";
        case "FPÖ":
            return "#0052FB";
        case "GRÜNE":
            return "#69B12E";
        case "NEOS":
            return "#E3257B";
        default:
            return "rgb(196, 180, 189)";
    }
}

export function setDelOnBubble(del: Delegate, circles2d: Bubble[][], fn: (party: string) => string) {
    if (del.seat_row == null || del.seat_col == null) {
        return
    }
    circles2d[del.seat_row-1][del.seat_col-1].del = del;
    // circles2d[del.seat_row-1][del.seat_col-1].color = partyToColor(del.party);
    circles2d[del.seat_row-1][del.seat_col-1].color = fn(del.party);
}

export function setupParliament(seats: number[], width: number, height: number, r: number): Bubble[][] {
    let circles2d: Bubble[][] = [];
    seats.forEach((seat, idx) => {
        circles2d.push(generateHalfCircle(seat, 70 + idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30: 0), width, height).map((circle) => {
            return {
                r,
                x: circle.x,
                y: circle.y,
                del: null,
                color: "rgb(196, 180, 189)",
                opacity: 0.2
            }
        }));
    });

    return circles2d;
}

function selectBubble(prev_selected: Bubble | null, selection: Bubble) {
    if (selection.del == null) {
        return;
    }
    if (prev_selected != null) {
        prev_selected.r = 6;
    }
    selection.r = +10.9;
}
