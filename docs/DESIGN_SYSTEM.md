# Xorion Design System Summary

This is a lightweight summary of the current Xorion visual system based on the live website and MVP wallet UI.

## Brand Direction

- Premium Web3 product
- Clean light surfaces with blue, purple, and cyan gradients
- High-trust wallet experience
- Soft glassmorphism, not heavy dark mode
- Rounded surfaces, smooth shadows, simple motion

## Core Visual Tokens

### Colors

- Background base: `#f5f8ff` to `#edf3ff`
- Text primary: `#131a33`
- Text secondary: `#65708f` to `#66718f`
- Border: `rgba(114, 135, 201, 0.14 - 0.16)`
- Primary blue: `#4f7cff`
- Primary purple: `#8d5dff`
- Primary cyan: `#39c9ff`
- Accent red: `#ff7a89`
- Success: `#18b37b` on website, `#79f7c4` in wallet surfaces

### Gradients

- Main brand gradient:
  `linear-gradient(135deg, #4f7cff 0%, #8d5dff 52%, #39c9ff 100%)`
- Soft panel glow:
  use low-opacity blue/purple/cyan radial or linear overlays

### Shadows

- Large shadow:
  `0 32px 80px rgba(92, 121, 209, 0.18)`
- Medium shadow:
  `0 18px 50px rgba(92, 121, 209, 0.12)`
- Use stronger shadows only for primary mockups or major hero cards

### Radius

- XL surfaces: `32px`
- Large cards: `24px` to `28px`
- Medium controls: `16px` to `18px`
- Pills/chips/buttons: `999px`

### Spacing

- Tight: `8px`
- Small: `12px`
- Medium: `18px`
- Card padding: `24px`
- Large section spacing: `28px` to `32px`
- Vertical section rhythm on website: roughly `52px` to `86px`

### Type

- Display headings:
  `clamp(3rem, 7vw, 6rem)` on website
  `clamp(2rem, 5vw, 4.6rem)` in wallet app
- Section headings:
  `clamp(2.2rem, 5vw, 4rem)`
- Body text:
  around `1rem` to `1.08rem`
- Eyebrows:
  uppercase, small, letter-spaced, blue-toned

## Reusable Components

### Buttons

- Primary:
  gradient fill, white text, pill or rounded shape, soft lift on hover
- Secondary/Ghost:
  white glass background, subtle border, dark text
- Outline:
  transparent fill with blue border for lower emphasis

### Cards

- Glass cards with:
  white translucent background, soft border, medium shadow, 24px+ radius
- Hero cards:
  optional gradient overlay and slightly stronger shadow
- Metric cards:
  simple label, strong number, muted supporting copy

### Pills / Chips

- Used for:
  status, network labels, proof points, small actions
- Shape:
  full pill radius, soft white background, subtle border

### Navigation

- Clean, simple, low-noise
- Active states should use soft gradient or glow treatment
- Keep labels explicit and beginner-friendly

## Motion Rules

- Use subtle hover lift only
- Use gentle reveal transitions
- Respect `prefers-reduced-motion`
- Do not rely on motion for comprehension

## Responsiveness Rules

- Collapse multi-column sections into one column on tablet/mobile
- Full-width buttons on small screens
- Keep hero actions stacked on mobile
- Reduce decorative density before reducing content clarity

## Performance Rules

- Prefer CSS gradients over heavy assets
- Keep JavaScript small and progressive
- Use `content-visibility` for below-the-fold website sections where safe
- Avoid heavy frontend dependencies for visuals

## What Future Xorion Pages Should Reuse

- The same blue/purple/cyan gradient family
- The same glass card treatment
- The same button hierarchy: primary, secondary, outline
- The same heading and eyebrow pattern
- The same radius scale
- The same shadow scale
- The same muted text color and border treatment
- The same status-chip language for trust and network visibility
