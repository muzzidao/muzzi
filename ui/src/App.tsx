// src/App.tsx
import React, { useEffect, useState } from 'react';
import KinodeEncryptorApi from '@kinode/client-api';
import GridLayout, { Layout } from 'react-grid-layout';
import { LayoutManager } from './utils/layoutManager';

import Player from "./components/panels/Player";
import Search from "./components/panels/Search";
import Profile from "./components/panels/Profile";
import Playlist from "./components/panels/Playlist";
import Discovery from "./components/panels/Discovery";
import Friends from "./components/panels/Friends";
import Downloader from "./components/panels/Downloader";
import Settings from "./components/panels/Settings";


const App = () => {
  const [ws, setWs] = useState<KinodeEncryptorApi | null>(null);
  const [layout, setLayout] = useState<Layout[]>([
    { i: 'player', x: 0, y: 0, w: 2, h: 2 },
    { i: 'search', x: 2, y: 0, w: 2, h: 2 },
    { i: 'profile', x: 4, y: 0, w: 2, h: 2 },
    { i: 'playlist', x: 0, y: 2, w: 2, h: 2 },
    { i: 'discovery', x: 2, y: 2, w: 2, h: 2 },
    { i: 'friends', x: 4, y: 2, w: 2, h: 2 },
    { i: 'downloader', x: 0, y: 4, w: 3, h: 2 },
    { i: 'settings', x: 3, y: 4, w: 3, h: 2 }
  ]);

  const handleSplitH = (itemId: string) => {
    setLayout(LayoutManager.splitHorizontally(layout, itemId));
  };

  const handleSplitV = (itemId: string) => {
    setLayout(LayoutManager.splitVertically(layout, itemId));
  };

  const handleClose = (itemId: string) => {
    setLayout(LayoutManager.remove(layout, itemId));
  };

  const renderPanel = (id: string) => {
    const commonProps = {
      onSplit: () => handleSplitH(id),
      onStack: () => {}, // Implement stacking logic
      onClose: () => handleClose(id)
    };

    switch (id.split('-')[0]) { // Handle split panels by checking base id
      case 'player': return <Player {...commonProps} />;
      case 'search': return <Search {...commonProps} />;
      case 'profile': return <Profile {...commonProps} />;
      case 'playlist': return <Playlist {...commonProps} />;
      case 'discovery': return <Discovery {...commonProps} />;
      case 'friends': return <Friends {...commonProps} />;
      case 'downloader': return <Downloader {...commonProps} />;
      case 'settings': return <Settings {...commonProps} />;
      default: return null;
    }
  };

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
        onLayoutChange={setLayout}
      >
        {layout.map((item) => (
          <div key={item.i}>
            {renderPanel(item.i)}
          </div>
        ))}
      </GridLayout>
    </div>
  );
};

export default App;


//import { useState, useEffect } from "react";
//import KinodeClientApi from "@kinode/client-api";
//import styles from "./App.module.css";
//
/////
//import GridLayout from 'react-grid-layout';
//import Player from "./components/panels/Player";
//import Search from "./components/panels/Search";
//import Profile from "./components/panels/Profile";
//import Playlist from "./components/panels/Playlist";
//import Discovery from "./components/panels/Discovery";
//import Friends from "./components/panels/Friends";
//import Downloader from "./components/panels/Downloader";
//import Settings from "./components/panels/Settings";
//
/////
//
//const BASE_URL = import.meta.env.BASE_URL;
//if (window.our) window.our.process = BASE_URL?.replace("/", "");
//
//const PROXY_TARGET = `${
//  import.meta.env.VITE_NODE_URL || "http://localhost:8080"
//}${BASE_URL}`;
//
//// This env also has BASE_URL which should match the process + package name
//const WEBSOCKET_URL = import.meta.env.DEV
//  ? `${PROXY_TARGET.replace("http", "ws")}`
//  : undefined;
//
//interface Friend {
//  name: string;
//  status: "Online" | "Offline";
//}
//
//interface Download {
//  name: string;
//  progress: number;
//}
//
//function App() {
//  const [nodeConnected, setNodeConnected] = useState(true);
//  const [api, setApi] = useState<KinodeClientApi | undefined>();
//
//  const [tracks, setTracks] = useState<string[]>([]);
//  const [friends, setFriends] = useState<Friend[]>([]);
//  const [downloads, setDownloads] = useState<Download[]>([]);
//  const [isPlaying, setIsPlaying] = useState(false);
//  const [searchQuery, setSearchQuery] = useState("");
//
//  useEffect(() => {
//    // Mock data initialization
//    const mockTracks = ["Track 1", "Track 2", "Track 3"];
//    const mockFriends: Friend[] = [
//      { name: "Friend 1", status: "Online" },
//      { name: "Friend 2", status: "Offline" },
//    ];
//    const mockDownloads: Download[] = [
//      { name: "Track 1.mp3", progress: 45 },
//      { name: "Track 2.mp3", progress: 78 },
//    ];
//
//    setTracks(mockTracks);
//    setFriends(mockFriends);
//    setDownloads(mockDownloads);
//  }, []);
//
//    const layout = [
//    { i: 'player', x: 0, y: 0, w: 2, h: 2 },
//    { i: 'search', x: 2, y: 0, w: 2, h: 2 },
//    { i: 'playlist', x: 4, y: 0, w: 2, h: 2 },
//    { i: 'friends', x: 0, y: 2, w: 2, h: 2 },
//    { i: 'downloader', x: 2, y: 2, w: 2, h: 2 },
//  ];
//
//  const handlePlayPause = () => {
//    setIsPlaying((prev) => !prev);
//  };
//
//  const handlePrev = () => {
//    // Implement previous track functionality
//    console.log("Previous track");
//  };
//
//  const handleNext = () => {
//    // Implement next track functionality
//    console.log("Next track");
//  };
//
//  const filteredTracks = tracks.filter((track) =>
//    track.toLowerCase().includes(searchQuery.toLowerCase())
//  );
//
//  
//  return (
//  <div style={{ 
//    padding: '20px',
//    background: '#1a1a1a',
//    minHeight: '100vh',
//    color: '#fff'
//  }}>
//    <GridLayout
//      className="layout"
//      layout={layout}
//      cols={6}
//      rowHeight={100}
//      width={1200}
//      draggableHandle=".panel-header"
//    >
//      <div key="player"><Player /></div>
//      <div key="search"><Search /></div>
//      <div key="profile"><Profile /></div>
//      <div key="playlist"><Playlist /></div>
//      <div key="discovery"><Discovery /></div>
//      <div key="friends"><Friends /></div>
//      <div key="downloader"><Downloader /></div>
//      <div key="settings"><Settings /></div>
//    </GridLayout>
//  </div>
//  );
//
//}
//export default App;
//