# Description

This VSCode extension aims to provide a better language support for the smart-contract language "solidity". This project was initially created one year and a half ago when there was almost no extension besides Juan Blanco's one.  
Check out our [discord](https://discord.gg/vFXVFwqtHT) and our [twitter](https://twitter.com/osmiumtoolchain) for important announcements.

# Usage
The extension can be dowloaded directly from the VSCode Marketplace by searching for "Osmium solidity" or by using the direct ID [osmiumtoolchains.osmium-solidity-extension](https://marketplace.visualstudio.com/items?itemName=OsmiumToolchains.osmium-solidity-extension).  

When openning a solidity project/file, all the features will be enabled by default. 
  

# Project structure
## libs
This folder contains a set of generic rust libraries to manipulate solidity related data such as project and source files. Thoses libraries can be used in your projects.  

## servers
This directory contains all the LSP and DAP servers necessary for all the extension's features. Thoses servers can also be used for another IDE integration (which is not on our roadmap).

## sidebar
The frontend project related to the sidebar panel of the extension.

## vscode
This folders contains the proper extension project scaffold that launches the server and provide some features in typescript