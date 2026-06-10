import { TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";

export default function ZoomView({ image_path }: { image_path: string }) {
  return (
    <div className="p-3 flex flex-grow-1 bg-black justify-center items-center h-full max-h-full">
      <div style={{ aspectRatio: 3 / 2, maxHeight: "100%" }}>
        <TransformWrapper
          initialScale={1}
          minScale={1}
          maxScale={32}
          wheel={{ step: 0.01 }}
          limitToBounds={true}
          velocityAnimation={{ disabled: true }}
          zoomAnimation={{ disabled: true }}
          autoAlignment={{ animationTime: 0 }}
        >
          <TransformComponent
            wrapperClass="!w-full !h-full flex items-center justify-center"
            contentClass="!w-full !h-full flex items-center justify-center"
          >
            <img style={{ width: "100%", maxHeight: "100%" }} src={image_path} />
          </TransformComponent>
        </TransformWrapper>
      </div>
    </div>
  );
}
