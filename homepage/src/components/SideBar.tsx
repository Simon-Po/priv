export default function SideBar({ NavBarHeight }) {
  return (
    <div
      className=" w-12 bg-[#B4B4B8] border-r-2 border-slate-700"
      style={{ height: "calc(100vh - " + NavBarHeight + "rem)" }}
    ></div>
  );
}
