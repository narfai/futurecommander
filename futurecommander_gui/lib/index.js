const { app, BrowserWindow } = require('electron');

function createWindow () {
    let window = new BrowserWindow({ width: 800, height: 600 });

    window.loadFile('lib/index.html');

    window.webContents.openDevTools();

    window.on('closed', () => {
        win = null
    })
}

app.on('ready', createWindow);
