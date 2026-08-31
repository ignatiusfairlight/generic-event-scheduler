# Generic Event Scheduler

<h3 align="center"><em>"A glorified spreadsheet for managing events."</em></h3>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white" />
  <img src="https://img.shields.io/badge/SvelteKit-FF3E00?style=for-the-badge&logo=svelte&logoColor=white" />
  <img src="https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white" />
</p>

## Project Overview

As the tagline suggests, what this app is capable of can all be done in a spreadsheet. The difference is
that you don't have to go through the hoops of "Data validation" or "Conditional Formatting" or even worse;
"Visual Basic Commands". This app was intended to help a local diocese in managing their events, which is a
small part of a bigger effort to digitize their records and day-to-day operations. What was once considered a
complicated project (by me) ended up as a simplified solution to the problem; a local event scheduler, no servers, no remote hosting, all data saved locally. Of course for the diocese, I intend to improve
this app further to customize their needs specifically, so in the meantime I'm letting this barebones version
in the public as case study for my own reference in the future. 

## Functionalities
- Create, edit, delete events
- View events on a calendar
- Filter events by title, date range, and approval status
- Approve or reject events
- Option to color code events
- Archive past approved events
- (Coming Soon) Render event form (export as PNG)

## Prerequisites

### For users
Windows — Download the .exe file and run the installer. The .exe file can be found in `Releases`.

### For local development
What you need:
- Node.js
- pnpm
- Rust toolchain (via rustup)

Then:
```bash
pnpm install
pnpm tauri dev
```

## How It Works
<!-- Add pictures here later -->
- Open the app → lands on Dashboard
- Dashboard shows the calendar, upcoming approved events, and events pending approval
- Sidebar navigates between Dashboard, Event Details, and Archives
- Event Details is where most operations happen — create, edit, delete, approve/reject events <!-- please expand this further -->
- Archives stores past approved events for reference

## Technical Overview
By running SvelteKit in SPA mode via adapter-static, I can produce static HTML/CSS/JS files that Tauri loads directly — no web server required, making this a serverless desktop application.

This version of the application have SvelteKit handles the UI, filtering, and pagination on the frontend. In my opinion, these
should have been written in the backend instead to make the application more robust. However, since this is more of a proof of
concept + exploratory project in Tauri + my first attempt at making a desktop application, I will leave it like this for now.

SvelteKit then communicate with Tauri backend written in Rust via Tauri's invoke command which you can see if you go through the event CRUD functions. And all that is left is for Rust to interact with SQLite for most of the CRUD operations. The best part of using SQLite is that user does not need a remote server to host their database because of how light SQLite is in nature.
The only downside for it right now is if something happens to the database, then there are no backups for it (for now).

### Why Svelte/SvelteKit?
Svelte is my first Javascript frontend framework, and I intend to make it my only framework ever since. No offense to React or Vue, but Svelte runes make frontend development less dull than it already is right now. Is it as mature as the other two? No, and I am fine with it. For me at least, I find code written in Svelte is more readable than other frontend frameworks out there. Call it a skill issue or whatever, I'll take it and happy to live with it till the day I can no longer code.

### Why Tauri-Rust?
I don't like Electron. Plus Tauri has way smaller bundle size and Tauri apps in general does not hog RAM usage like Electron apps do (which means a lot in these days where RAM prices are way expensive than they should be).

## Lessons Learned
- Discovered Tauri as a method of turning web apps into desktop apps
- Figured out a serverless solution for a management software by using SQLite
- Learning Svelte 5 runes and reactivity patterns
- SPA mode with Superforms and handling form validation without a server
- Tanstack Table patterns like the createColumns closure
- Tauri's invoke/command pattern for frontend-backend communication
- GitHub Actions for CI/CD and automated builds

## Potential Improvement(s)
- Conflict detection (when events happening on the same location has overlapping time)
- Moving all the business logic (filtering, pagination, etc) to the Tauri side
- Using a different (better) UI library
- Add additional page/section for a dedicated configuration options (creating presets, auto color coding by location)
- Backup data via cloud storage
- Auto updater

<!-- ## Post Mortem -->
