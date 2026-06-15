import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://edithatogo.github.io',
  base: '/dnz/',
  integrations: [
    mdx(),
    sitemap(),
    starlight({
      title: 'DigitalNZ Integration Hub',
      description: 'Legal NZ documentation portal for DigitalNZ Integration Hub.',
      sidebar: [
        { label: 'Start', items: ['index', 'docs-tooling-audit'] },
      ],
    }),
  ],
});
