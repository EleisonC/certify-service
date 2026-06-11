/** @type {import('next').NextConfig} */
const nextConfig = {
  // Emit a self-contained server bundle (.next/standalone) so the Docker
  // runtime image only needs Node, not node_modules or the Next CLI.
  output: "standalone",
};

export default nextConfig;
