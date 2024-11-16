import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "./assets/vite.svg";
import KinodeClientApi from "@kinode/client-api";
import { SendSandboxMessage } from "./types/Sandbox";
import useSandboxStore from "./store/sandbox";
import styles from "./App.module.css";

const BASE_URL = import.meta.env.BASE_URL;
if (window.our) window.our.process = BASE_URL?.replace("/", "");

const PROXY_TARGET = `${
  import.meta.env.VITE_NODE_URL || "http://localhost:8080"
}${BASE_URL}`;

// This env also has BASE_URL which should match the process + package name
const WEBSOCKET_URL = import.meta.env.DEV
  ? `${PROXY_TARGET.replace("http", "ws")}`
  : undefined;

interface Friend {
  name: string;
  status: "Online" | "Offline";
}

interface Download {
  name: string;
  progress: number;
}

function App() {
  const [nodeConnected, setNodeConnected] = useState(true);
  const [api, setApi] = useState<KinodeClientApi | undefined>();

  const [tracks, setTracks] = useState<string[]>([]);
  const [friends, setFriends] = useState<Friend[]>([]);
  const [downloads, setDownloads] = useState<Download[]>([]);
  const [isPlaying, setIsPlaying] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    // Mock data initialization
    const mockTracks = ["Track 1", "Track 2", "Track 3"];
    const mockFriends: Friend[] = [
      { name: "Friend 1", status: "Online" },
      { name: "Friend 2", status: "Offline" },
    ];
    const mockDownloads: Download[] = [
      { name: "Track 1.mp3", progress: 45 },
      { name: "Track 2.mp3", progress: 78 },
    ];

    setTracks(mockTracks);
    setFriends(mockFriends);
    setDownloads(mockDownloads);
  }, []);

  const handlePlayPause = () => {
    setIsPlaying((prev) => !prev);
  };

  const handlePrev = () => {
    // Implement previous track functionality
    console.log("Previous track");
  };

  const handleNext = () => {
    // Implement next track functionality
    console.log("Next track");
  };

  const filteredTracks = tracks.filter((track) =>
    track.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className={styles.App}>
      <div className={styles.grid}>
        {/* Player Panel */}
        <div className={styles.panel}>
          <div className={styles.panelHeader}>Player</div>
          <div className={styles.panelContent}>
            <div className={styles.track}>
              Now Playing: {isPlaying ? "Playing" : "No track selected"}
            </div>
            <div className={styles.playerControls}>
              <button id="prev" onClick={handlePrev}>
                ◀◀
              </button>
              <button id="playPause" onClick={handlePlayPause}>
                {isPlaying ? "⏸️" : "▶️"}
              </button>
              <button id="next" onClick={handleNext}>
                ▶▶
              </button>
            </div>
            <div className={styles.progressBar}>
              <div
                className={styles.progress}
                style={{ width: isPlaying ? "30%" : "0%" }}
              ></div>
            </div>
          </div>
        </div>

        {/* Search Panel */}
        <div className={styles.panel}>
          <div className={styles.panelHeader}>Search</div>
          <div className={styles.panelContent}>
            <input
              type="text"
              placeholder="Search tracks..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className={styles.searchInput}
            />
            <div className={styles.track}>Search Results:</div>
            {filteredTracks.map((track, index) => (
              <div key={index} className={styles.track}>
                {track}
              </div>
            ))}
          </div>
        </div>

        {/* Playlist Panel */}
        <div className={styles.panel}>
          <div className={styles.panelHeader}>Playlist</div>
          <div className={`${styles.panelContent} ${styles.playlist}`}>
            {tracks.map((track, index) => (
              <div key={index} className={styles.track}>
                {track}
              </div>
            ))}
          </div>
        </div>

        {/* Friends Panel */}
        <div className={styles.panel}>
          <div className={styles.panelHeader}>Friends</div>
          <div className={`${styles.panelContent} ${styles.friends}`}>
            {friends.map((friend, index) => (
              <div key={index} className={styles.track}>
                {friend.name}
                <span
                  className={
                    friend.status === "Online" ? styles.online : styles.offline
                  }
                >
                  {friend.status}
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Downloads Panel */}
        <div className={styles.panel}>
          <div className={styles.panelHeader}>Downloads</div>
          <div className={`${styles.panelContent} ${styles.downloads}`}>
            {downloads.map((download, index) => (
              <div key={index} className={styles.track}>
                <div>{download.name}</div>
                <div className={styles.progressBar}>
                  <div
                    className={styles.progress}
                    style={{ width: `${download.progress}%` }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Settings Panel */}
        <div className={styles.panel}>
          <div className={styles.panelHeader}>Settings</div>
          <div className={styles.panelContent}>
            <div className={styles.track}>
              <label>
                Show Offline Friends
                <input type="checkbox" className={styles.checkbox} />
              </label>
            </div>
            <div className={styles.track}>
              <label>
                Enable Discovery
                <input type="checkbox" className={styles.checkbox} />
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
