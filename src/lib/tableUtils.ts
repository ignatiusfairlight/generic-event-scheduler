import type { CalendarDate } from "@internationalized/date";

export type Event = {
    id: number;
    title: string;
    start: string;
    end: string;
    location: string;
    person_in_charge: string;
    contact_num: string;
    is_approved: number;
    color: string;
}

export const statusMap: Record<number, string> = {
    0: "Pending",
    1: "Approved",
    2: "Rejected"
}

export function formatDate(datetime: string): string {
    const date = new Date(datetime);
    return date.toLocaleDateString('en-GB', {
        day: '2-digit',
        month: 'long',
        year: 'numeric'
    })
}

export function formatTime(datetime: string): string {
    const date = new Date(datetime);
    return date.toLocaleTimeString('en-US', {
        hour: 'numeric',
        minute: '2-digit',
        hour12: true
    })
}

export function calendarDateToString(date: CalendarDate): string {
    return `${date.year}-${String(date.month).padStart(2, '0')}-${String(date.day).padStart(2, '0')}`;
}
