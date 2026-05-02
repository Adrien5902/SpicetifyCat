# 🐞 Issues and PR
If you encounter some bug or wanna suggest a feature, feel free to open a PR or an Issue, or you could contact me on Discord

# 🎨 Creating your own theme
1. Fork the repo
2. Add your custom images under `themes/YOUR_THEME_NAME/background.png` and `themes/YOUR_THEME_NAME/liked_songs.png`
3. Edit `color.ini` and make a new section looking like this

   ```ini
    ; color.ini

    ; ... other themes
   
    [YOUR_THEME_NAME]
    ; each field should be six digits hex colors without # (e.g. b693ce)
    button = ______ ; color of the buttons in the app
    button-active = ______ ; kinda the same
    ```
4. Take a screenshot, place it under `themes/YOUR_THEME_NAME/preview.png` and add it to [THEMES.md](./THEMES.md) for others to see
5. Open a Pull request [here](https://github.com/Adrien5902/SpicetifyCat/pulls)
