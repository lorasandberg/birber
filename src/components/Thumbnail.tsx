export default function Thumbnail({ path }: { path: string }) {
  return (
    <div
      style={{
        width: "240px",
        height: "240px",
        border: "1px solid #dedede",
        backgroundColor: "#fff",
        padding: "1em",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <img style={{ maxWidth: "100%", maxHeight: "100%" }} src={path} />
    </div>
  );
}
