import React, { useEffect, useState } from 'react';
import KinodeEncryptorApi from '@kinode/client-api';
import GridLayout from 'react-grid-layout';

import Player from './components/panels/Player';
import Search from './components/panels/Search';
import Profile from './components/panels/Profile';
import Playlist from './components/panels/Playlist';
import Discovery from './components/panels/Discovery';
import Friends from './components/panels/Friends';
import Downloader from './components/panels/Downloader';
import Settings from './components/panels/Settings';

const App = () => {
  const [ws, setWs] = useState<KinodeEncryptorApi | null>(null);

  useEffect(() => {
    const api = new KinodeEncryptorApi({
      nodeId: window.our.node,
      processId: window.our.process,
      onMessage: (message) => {
        console.log('Received message:', message);
      },
    });

    setWs(api);
  }, []);

  const layout = [
    { i: 'player', x: 0, y: 0, w: 2, h: 2 },
    { i: 'search', x: 2, y: 0, w: 2, h: 2 },
    { i: 'profile', x: 4, y: 0, w: 2, h: 2 },
    { i: 'playlist', x: 0, y: 2, w: 2, h: 2 },
    { i: 'discovery', x: 2, y: 2, w: 2, h: 2 },
    { i: 'friends', x: 4, y: 2, w: 2, h: 2 },
    { i: 'downloader', x: 0, y: 4, w: 3, h: 2 },
    { i: 'settings', x: 3, y: 4, w: 3, h: 2 }
  ];

  return (
    <div style={{ 
      padding: '20px',
      background: '#1a1a1a',
      minHeight: '100vh',
      color: '#fff'
    }}>
      <GridLayout
        className="layout"
        layout={layout}
        cols={6}
        rowHeight={100}
        width={1200}
        draggableHandle=".panel-header"
      >
        <div key="player"><Player /></div>
        <div key="search"><Search /></div>
        <div key="profile"><Profile /></div>
        <div key="playlist"><Playlist /></div>
        <div key="discovery"><Discovery /></div>
        <div key="friends"><Friends /></div>
        <div key="downloader"><Downloader /></div>
        <div key="settings"><Settings /></div>
      </GridLayout>
    </div>
  );
};

export default App;



//import React, { useEffect, useState } from 'react';
//import GridLayout from 'react-grid-layout';
//import KinodeEncryptorApi from '@kinode/client-api';
//
//// Panel Components
//import { Player } from './components/panels/Player';
//import { Search } from './components/panels/Search';
//import { Profile } from './components/panels/Profile';
//import { Playlist } from './components/panels/Playlist';
//import { Discovery } from './components/panels/Discovery';
//import { Friends } from './components/panels/Friends';
//import { Downloader } from './components/panels/Downloader';
//import { Settings } from './components/panels/Settings';
//
//// Styles
//import 'react-grid-layout/css/styles.css';
//import 'react-resizable/css/styles.css';
//
//const App = () => {
//  const [ws, setWs] = useState<KinodeEncryptorApi | null>(null);
//
//  // Initialize WebSocket connection
//  useEffect(() => {
//    const api = new KinodeEncryptorApi({
//      uri: import.meta.env.DEV ? `ws://localhost:${import.meta.env.VITE_PORT}` : undefined,
//      nodeId: window.our.node,
//      processId: window.our.process,
//      onMessage: (message) => {
//        console.log('Received message:', message);
//        // Handle different message types here
//      },
//    });
//
//    setWs(api);
//  }, []);
//
//  // Define initial layout
//  const layout = [
//    { i: 'player', x: 0, y: 0, w: 2, h: 2 },
//    { i: 'search', x: 2, y: 0, w: 2, h: 2 },
//    { i: 'playlist', x: 4, y: 0, w: 2, h: 2 },
//    { i: 'friends', x: 0, y: 2, w: 2, h: 2 },
//    { i: 'downloader', x: 2, y: 2, w: 2, h: 2 },
//  ];
//
//  return (
//    <div className="h-screen bg-gray-900 text-white p-4">
//      <GridLayout 
//        className="layout"
//        layout={layout}
//        cols={6}
//        rowHeight={100}
//        width={1200}
//      >
//        <div key="player">
//          <Player />
//        </div>
//        <div key="search">
//          <Search />
//        </div>
//        <div key="playlist">
//          <Playlist />
//        </div>
//        <div key="friends">
//          <Friends />
//        </div>
//        <div key="downloader">
//          <Downloader />
//        </div>
//      </GridLayout>
//    </div>
//  );
//};
//
//export default App;
//
//
////import { useState, useEffect, useCallback } from "react";
////import reactLogo from "./assets/react.svg";
////import viteLogo from "./assets/vite.svg";
////import KinodeClientApi from "@kinode/client-api";
////import "./App.css";
////import { SendMuzziPlayerMessage } from "./types/MuzziPlayer";
////import useMuzziPlayerStore from "./store/muzzi_player";
////
////const BASE_URL = import.meta.env.BASE_URL;
////if (window.our) window.our.process = BASE_URL?.replace("/", "");
////
////const PROXY_TARGET = `${(import.meta.env.VITE_NODE_URL || "http://localhost:8080")}${BASE_URL}`;
////
////// This env also has BASE_URL which should match the process + package name
////const WEBSOCKET_URL = import.meta.env.DEV
////  ? `${PROXY_TARGET.replace('http', 'ws')}`
////  : undefined;
////
////function App() {
////  const { muzzi_players, addMessage, set } = useMuzziPlayerStore();
////  const [selectedMuzziPlayer, setSelectedMuzziPlayer] = useState("New MuzziPlayer");
////
////  const [target, setTarget] = useState("");
////  const [message, setMessage] = useState("");
////  const [nodeConnected, setNodeConnected] = useState(true);
////  const [api, setApi] = useState<KinodeClientApi | undefined>();
////
////  useEffect(() => {
////    // Get message history using http
////    fetch(`${BASE_URL}/messages`)
////      .then((response) => response.json())
////      .then((data) => {
////        set({ muzzi_players: { ...(data?.History?.messages || {}), "New MuzziPlayer": [] } });
////      })
////      .catch((error) => console.error(error));
////
////    // Connect to the Kinode via websocket
////    console.log('WEBSOCKET URL', WEBSOCKET_URL)
////    if (window.our?.node && window.our?.process) {
////      const api = new KinodeClientApi({
////        uri: WEBSOCKET_URL,
////        nodeId: window.our.node,
////        processId: window.our.process,
////        onOpen: (_event, _api) => {
////          console.log("Connected to Kinode");
////          // api.send({ data: "Hello World" });
////        },
////        onMessage: (json, _api) => {
////          console.log('WEBSOCKET MESSAGE', json)
////          try {
////            const data = JSON.parse(json);
////            console.log("WebSocket received message", data);
////            const [messageType] = Object.keys(data);
////            if (!messageType) return;
////
////            if (messageType === "NewMessage") {
////              addMessage(data.NewMessage);
////            }
////          } catch (error) {
////            console.error("Error parsing WebSocket message", error);
////          }
////        },
////      });
////
////      setApi(api);
////    } else {
////      setNodeConnected(false);
////    }
////  }, []);
////
////  const startMuzziPlayer = useCallback(
////    (event) => {
////      event.preventDefault();
////
////      if (!api || !target) return;
////
////      const newMuzziPlayers = { ...muzzi_players };
////      newMuzziPlayers[target] = [];
////
////      setSelectedMuzziPlayer(target);
////      set({ muzzi_players: newMuzziPlayers });
////
////      setTarget("");
////    },
////    [api, muzzi_players, set, setSelectedMuzziPlayer, target, setTarget]
////  );
////
////  const sendMessage = useCallback(
////    async (event) => {
////      event.preventDefault();
////
////      if (!api || !message || !selectedMuzziPlayer) return;
////
////      // Create a message object
////      const data = {
////        Send: {
////          target: selectedMuzziPlayer,
////          message,
////        },
////      } as SendMuzziPlayerMessage;
////
////      // Send a message to the node via websocket
////      // UNCOMMENT THE FOLLOWING 2 LINES to send message via websocket
////      // api.send({ data });
////      // setMessage("");
////
////      // Send a message to the node via HTTP request
////      // IF YOU UNCOMMENTED THE LINES ABOVE, COMMENT OUT THIS try/catch BLOCK
////      try {
////        const result = await fetch(`${BASE_URL}/messages`, {
////          method: "POST",
////          body: JSON.stringify(data),
////        });
////
////        if (!result.ok) throw new Error("HTTP request failed");
////
////        // Add the message if the POST request was successful
////        const newMuzziPlayers = { ...muzzi_players };
////        newMuzziPlayers[selectedMuzziPlayer].push({ author: window.our?.node, content: message });
////        set({ muzzi_players: newMuzziPlayers });
////        setMessage("");
////      } catch (error) {
////        console.error(error);
////      }
////    },
////    [api, message, setMessage, selectedMuzziPlayer, muzzi_players, set]
////  );
////
////  return (
////    <div style={{ width: "100%" }}>
////      <div style={{ position: "absolute", top: 4, left: 8 }}>
////        ID: <strong>{window.our?.node}</strong>
////      </div>
////      {!nodeConnected && (
////        <div className="node-not-connected">
////          <h2 style={{ color: "red" }}>Node not connected</h2>
////          <h4>
////            You need to start a node at {PROXY_TARGET} before you can use this UI
////            in development.
////          </h4>
////        </div>
////      )}
////      <h2>Simple MuzziPlayer on Kinode</h2>
////      <div className="card">
////        <div
////          style={{
////            display: "flex",
////            flexDirection: "row",
////            border: "1px solid gray",
////          }}
////        >
////          <div
////            style={{ flex: 1, borderRight: "1px solid gray", padding: "1em" }}
////          >
////            <h3 style={{ marginTop: 0 }}>MuzziPlayers</h3>
////            <ul>
////              {Object.keys(muzzi_players).map((muzzi_playerId) => (
////                <li key={muzzi_playerId}>
////                  <button
////                    onClick={() => setSelectedMuzziPlayer(muzzi_playerId)}
////                    className={`muzzi-player-button ${selectedMuzziPlayer === muzzi_playerId ? "selected" : ""}`}
////                  >
////                    {muzzi_playerId}
////                  </button>
////                </li>
////              ))}
////            </ul>
////          </div>
////          <div
////            style={{
////              display: "flex",
////              flexDirection: "column",
////              justifyContent: "space-between",
////              flex: 2,
////              padding: "1em",
////            }}
////          >
////            <h3 style={{ marginTop: 0, textAlign: 'left' }}>{selectedMuzziPlayer}</h3>
////            {selectedMuzziPlayer === "New MuzziPlayer" ? (
////              <form
////                onSubmit={startMuzziPlayer}
////                style={{ display: "flex", flexDirection: "column" }}
////              >
////                <label
////                  style={{ fontWeight: 600, alignSelf: "flex-start" }}
////                  htmlFor="target"
////                >
////                  Node
////                </label>
////                <input
////                  style={{
////                    padding: "0.25em 0.5em",
////                    fontSize: "1em",
////                    marginBottom: "1em",
////                  }}
////                  type="text"
////                  id="target"
////                  value={target}
////                  onChange={(event) => setTarget(event.target.value)}
////                />
////                <button type="submit">Start New MuzziPlayer</button>
////              </form>
////            ) : (
////              <div>
////                <div>
////                  <ul className="message-list">
////                    {selectedMuzziPlayer &&
////                      muzzi_players[selectedMuzziPlayer]?.map((message, index) => (
////                        <li key={index} className={`message ${message.author === window.our?.node ? 'ours' : ''}`}>
////                          {message.content}
////                        </li>
////                      ))}
////                  </ul>
////                </div>
////                <form
////                  onSubmit={sendMessage}
////                  style={{
////                    display: "flex",
////                    flexDirection: "column",
////                    width: "100%",
////                  }}
////                >
////                  <div className="input-row">
////                    <input
////                      type="text"
////                      id="message"
////                      placeholder="Message"
////                      value={message}
////                      onChange={(event) => setMessage(event.target.value)}
////                      autoFocus
////                    />
////                    <button type="submit">Send</button>
////                  </div>
////                </form>
////              </div>
////            )}
////          </div>
////        </div>
////        <p>
////          Edit <code>src/App.tsx</code> and save to test Hot Module Reloading
////          (HMR)
////        </p>
////      </div>
////      <div>
////        <a href="https://vitejs.dev" target="_blank">
////          <img src={viteLogo} className="logo" alt="Vite logo" />
////        </a>
////        <a href="https://react.dev" target="_blank">
////          <img src={reactLogo} className="logo react" alt="React logo" />
////        </a>
////      </div>
////      <p className="read-the-docs">
////        Click on the Vite and React logos to learn more
////      </p>
////    </div>
////  );
////}
////
////export default App;
////