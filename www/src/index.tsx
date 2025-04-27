import './style.css'
import ReactDOM from 'react-dom/client'
import { Canvas } from '@react-three/fiber'
import Experience from './Experience'
import RemoteReadme from './RemoteReadme'

const container = document.querySelector('#root');
if (container) {
    const root = ReactDOM.createRoot(container);
    root.render(
        <>
            <Canvas
                className="r3f"
                camera={{
                    fov: 45,
                    near: 0.1,
                    far: 2000,
                    position: [-3, 1.5, 4]
                }}
                style={{ position: 'sticky', top: 0, left: 0, width: '100vw', height: '100vh', zIndex: 1, pointerEvents: 'auto' }}
            >
                <Experience />
            </Canvas>
            <RemoteReadme />
        </>
    );
} else {
    console.error("#root element not found in the document.");
}