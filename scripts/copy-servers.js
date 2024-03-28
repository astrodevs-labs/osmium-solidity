const fs = require('fs');

const outputFolder = __dirname + '/../vscode/dist';

fs.readdir('servers', { withFileTypes: true }, (err, serversEntries) => {
  if (err) {
    console.error(err);
    return;
  }

  console.log('Server directories: ', serversEntries.map(file => file.name));

  const serversDirectories = serversEntries.filter(file => file.isDirectory()).map(file => file.name);
  serversDirectories.forEach((serverDir) => {
    // list release and debug targets
    fs.readdir(`servers/${serverDir}/target`, { withFileTypes: true }, (err, entries) => {
      if (err) {
        console.error(err);
        return;
      }
      const directories = entries.filter(directory => directory.isDirectory()).map(directory => directory.name);
      const targetDirectories = directories.filter(directory => directory == "release" || directory == "debug");
      const typeDir = targetDirectories.find(dir => dir == "release") || targetDirectories[0];
      // list server binaries in target directory
      fs.readdir(`servers/${serverDir}/target/${typeDir}`, { withFileTypes: true }, (err, entries) => {
        if (err) {
            console.error(err);
            return;
          }
        const files = entries.filter(file => file.isFile()).map(file => file.name);
        const serverBinary = files.filter(file => file.endsWith('-server') || file.endsWith('-server.exe'));
        console.log('Copying server binary to vscode/dist', serverBinary);
        fs.copyFile(`servers/${serverDir}/target/${typeDir}/${serverBinary}`, `${outputFolder}/${serverBinary}`, (err) => {
            if (err) {
            console.error(err);
            }
        });
      });
    });
  });
});