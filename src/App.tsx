import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'
import ScreenshotCanvas from '@/components/ScreenshotCanvas'
import ToolPanel from '@/components/ToolPanel'
import HistoryPanel from '@/components/HistoryPanel'

function App() {
  const [isCapturing, setIsCapturing] = useState(false)
  const [screenshots, setScreenshots] = useState<any[]>([])
  const [selectedImage, setSelectedImage] = useState<string | null>(null)

  useEffect(() => {
    // Load screenshot history on mount
    loadHistory()
  }, [])

  const loadHistory = async () => {
    try {
      // TODO: Implement history loading from backend
      console.log('Loading screenshot history...')
    } catch (error) {
      console.error('Failed to load history:', error)
    }
  }

  const startScreenshot = async () => {
    try {
      setIsCapturing(true)
      await invoke('trigger_screenshot')
    } catch (error) {
      console.error('Failed to start screenshot:', error)
      setIsCapturing(false)
    }
  }

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>VibeShot</h1>
        <p>Smart Screenshot Tool with AI Censoring</p>
      </header>

      <div className="app-content">
        <div className="main-area">
          {isCapturing ? (
            <ScreenshotCanvas onCapture={(imageData) => {
              setSelectedImage(imageData)
              setIsCapturing(false)
            }} />
          ) : selectedImage ? (
            <div className="image-editor">
              <img src={selectedImage} alt="Screenshot" />
              <ToolPanel imageData={selectedImage} />
            </div>
          ) : (
            <div className="welcome-area">
              <div className="welcome-content">
                <h2>Welcome to VibeShot</h2>
                <p>Take smart screenshots with AI-powered censoring</p>
                <button 
                  className="btn-primary"
                  onClick={startScreenshot}
                >
                  📸 Start Screenshot (Alt+Shift+S)
                </button>
              </div>
            </div>
          )}
        </div>

        <aside className="sidebar">
          <HistoryPanel 
            screenshots={screenshots}
            onSelectScreenshot={(id) => {
              // Load selected screenshot
              console.log('Selected screenshot:', id)
            }}
          />
        </aside>
      </div>
    </div>
  )
}

export default App
