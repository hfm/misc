# Pinboard Saver Chrome Extension

A Chrome extension to save bookmarks to Pinboard.in with one click.

## Features

- Click extension icon to show popup
- Auto-populate current page URL and title
- Add description, tags, and private settings
- Save and reuse Pinboard API token

## Installation

1. Open `chrome://extensions/` in Chrome
2. Enable "Developer mode"
3. Click "Load unpacked"
4. Select this folder

## Usage

1. Get your API token from Pinboard.in (https://pinboard.in/settings/password)
2. Click the extension icon on any page you want to bookmark
3. Enter your API token on first use
4. Edit description and tags as needed
5. Click "Save"

## File Structure

- `manifest.json` - Extension configuration
- `popup.html` - Popup UI
- `popup.css` - Styles
- `popup.js` - Main functionality
- `icon*.png` - Extension icons