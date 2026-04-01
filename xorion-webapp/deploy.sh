#!/bin/bash

# Xorion WebApp Deployment Script
# Deploys to Vercel/Netlify for free hosting

set -e

echo "🚀 Xorion WebApp Deployment Script"
echo "=================================="

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "❌ Trunk not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://trunk.binary.dev/install | sh
    source $HOME/.cargo/env
fi

echo "✅ Building for WASM..."
trunk build --release

echo "✅ Build complete! Output in ./dist/"
echo ""
echo "📁 Deploy options:"
echo ""
echo "Option 1: Vercel (Recommended)"
echo "  cd dist && vercel --prod"
echo ""
echo "Option 2: Netlify"
echo "  netlify deploy --prod --dir=dist"
echo ""
echo "Option 3: GitHub Pages"
echo "  git subtree push --prefix dist origin gh-pages"
echo ""
echo "Option 4: Manual upload"
echo "  Upload ./dist folder to any static hosting"
echo ""

# Auto-deploy to Vercel if available
if command -v vercel &> /dev/null; then
    read -p "Deploy to Vercel now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cd dist && vercel --prod
    fi
else
    echo "💡 Install Vercel CLI for one-click deployment:"
    echo "   npm install -g vercel"
fi

echo ""
echo "✅ Done!"
