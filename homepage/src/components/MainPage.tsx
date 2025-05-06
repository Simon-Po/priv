import NavBar from "@/components/NavBar";
import SideBar from "@/components/SideBar";
import BlogCard from "@/components/BlogCard";

export default function MainPage() {
  return (
    <div className="bg-[#C7C8CC]">
      <NavBar height={3.5}></NavBar>
      <div className="flex flex-row">
        <SideBar NavBarHeight={3.5}></SideBar>
        <div className="flex justify-center items-center w-screen ">
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-10 p-4">
            {[...Array(10)].map((_, index) => (
              <BlogCard key={index} content="Helllooooo">
                This is blog {index}
              </BlogCard>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
