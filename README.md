# DarkDayArch

An Architecture helper and mod content Designer for Cataclysm-DarkDaysAhead.

## Product Introduction

[Diagrams about architecture and UI structure](docs/diagrams), it uses draw.io (diagrams.net), so you can install drawio plugin of VSCode to open it.

## Development

`npm run` following scripts, for example, `npm run tauri:dev`

```shell
ui:start # run dev mode of create-react-app
ui:build
ui:test
tauri:dev # run ui:start as defined in `src-tauri/tauri.conf.json`, and start dev mode of tauri
tauri:build
tauri:help
```

### Open in specific browser

Create a `.env.local` in project root, and add this to `.env.local` as [create-react-app: How do I “npm start” with a specific browser?](https://stackoverflow.com/questions/51706882/create-react-app-how-do-i-npm-start-with-a-specific-browser):

```env
BROWSER="firefox"
```

### How to debug rust code

[App Debugging - tauri.studio/en/docs](https://tauri.studio/en/docs/usage/development/debugging/)
