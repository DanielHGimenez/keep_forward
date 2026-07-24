import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';

import styles from './index.module.css';

const REPO = 'https://github.com/DanielHGimenez/keep_forward';
const RELEASES = `${REPO}/releases/latest`;

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <img src="/keep_forward/img/Soldier Command Poster.png"/>
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <p className={styles.heroPitch}>
          Auto-walk forward in games without holding <kbd>W</kbd>. Bind your own
          hotkey, press it to toggle W held down, press W to stop.
        </p>
        <div className={styles.buttons}>
          <Link className="button button--secondary button--lg" to="/docs/intro">
            Get Started
          </Link>
          <Link
            className="button button--outline button--secondary-light button--lg"
            href={REPO}>
            GitHub
          </Link>
        </div>
      </div>
    </header>
  );
}

function TerminalDemo() {
  return (
    <section className={styles.section}>
      <div className="container">
        <Heading as="h2" className="text--center">
          See it run
        </Heading>
        <div className={styles.terminal}>
          <div className={styles.terminalBar}>
            <span />
            <span />
            <span />
          </div>
          <pre className={styles.terminalBody}>
            <code>{`$ keep_forward
keep_forward: hold up to 3 keys together then release to set your hotkey.
  press it to toggle holding W, Ctrl+C to quit.
  macOS: grant Accessibility permission. Linux: X11 only (not Wayland).

hotkey set to {ControlLeft, F8} pressed in any order. Press it to toggle holding W.
holding W: true
holding W: false (W pressed)`}</code>
          </pre>
        </div>
      </div>
    </section>
  );
}

type Step = {n: number; title: string; body: ReactNode};

const STEPS: Step[] = [
  {n: 1, title: 'Run it', body: <>Launch the binary from anywhere. No install, no config.</>},
  {
    n: 2,
    title: 'Set your hotkey',
    body: <>Hold up to three (or less) keys together and release &mdash; that combo is bound.</>,
  },
  {
    n: 3,
    title: 'Press to toggle',
    body: <>Tap your hotkey and <kbd>W</kbd> stays held down. Tap it again to release.</>,
  },
  {
    n: 4,
    title: 'Tap W to stop',
    body: <>Pressing <kbd>W</kbd> yourself instantly turns holding off.</>,
  },
];

function HowItWorks() {
  return (
    <section className={clsx(styles.section, styles.altSection)}>
      <div className="container">
        <Heading as="h2" className="text--center">
          How it works
        </Heading>
        <div className="row">
          {STEPS.map((s) => (
            <div key={s.n} className="col col--3">
              <div className={styles.step}>
                <span className={styles.stepNum}>{s.n}</span>
                <Heading as="h3">{s.title}</Heading>
                <p>{s.body}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

type Download = {os: string; target: string};

const DOWNLOADS: Download[] = [
  {os: '🪟 Windows', target: 'x86_64-pc-windows-msvc'},
  {os: '🍎 macOS (Apple Silicon)', target: 'aarch64-apple-darwin'},
  {os: '🍎 macOS (Intel)', target: 'x86_64-apple-darwin'},
  {os: '🐧 Linux', target: 'x86_64-unknown-linux-gnu'},
];

function Downloads() {
  return (
    <section className={styles.section}>
      <div className="container text--center">
        <Heading as="h2">Download</Heading>
        <p>Prebuilt binaries for every supported platform:</p>
        <div className={styles.downloadGrid}>
          {DOWNLOADS.map((d) => (
            <Link
              key={d.target}
              className="button button--primary button--lg"
              href={RELEASES}>
              <span className={styles.downloadOs}>{d.os}</span>
              <span className={styles.downloadTarget}>{d.target}</span>
            </Link>
          ))}
        </div>
        <p className={styles.note}>
          macOS needs Accessibility permission (System Settings → Privacy &amp;
          Security → Accessibility). Linux is X11 only, not Wayland.
        </p>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={siteConfig.title}
      description="Toggle auto-hold W in games with a hotkey — a tiny cross-platform Rust tool.">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <TerminalDemo />
        <HowItWorks />
        <Downloads />
      </main>
    </Layout>
  );
}
