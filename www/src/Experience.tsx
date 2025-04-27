import { Environment, Html, PresentationControls, useGLTF } from '@react-three/drei'

export default function Experience() {
    const computer = useGLTF('./computer.glb')

    return (
        <>
            <Environment preset="city" />
            <PresentationControls
                global
                snap={true}
                rotation={[0, -0.65, 0]}
            >
                <primitive
                    object={computer.scene}
                    scale={0.15}
                    position-y={0.2}
                >
                    <Html
                        occlude="blending"
                        transform
                        wrapperClass="htmlScreen"
                        distanceFactor={1.2}
                        position={[0.1, 1.8, 5]}
                        scale={2.52}
                    >
                       <iframe src="/yo-terminal.html"></iframe>
                    </Html>
                </primitive>
            </PresentationControls>
        </>
    )
}