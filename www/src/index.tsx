import './style.css'
import ReactDOM from 'react-dom/client'
import { Canvas } from '@react-three/fiber'
import Experience from './Experience'


const container = document.querySelector('#root');
if (container) {
    const root = ReactDOM.createRoot(container);
    root.render(
        <Canvas
            camera={{
                fov: 45,
                near: 0.1,
                far: 2000,
                position: [-3, 1.5, 4]
            }}
        >
            <Experience />
        </Canvas>
    );
} else {
    console.error("#root element not found in the document.");
}