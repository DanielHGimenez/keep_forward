import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  icon: string;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Your own hotkey',
    icon: '⌨️',
    description: (
      <>
        Hold up to three keys together, release, and that combo is your toggle.
        Matched in any order &mdash; no config files, set it live on first run.
      </>
    ),
  },
  {
    title: 'Works in real games',
    icon: '🎮',
    description: (
      <>
        On Windows it injects the <code>W</code> hardware scancode via{' '}
        <code>SendInput</code>, so DirectInput / Raw Input games actually
        register the key &mdash; not just virtual-key events they ignore.
      </>
    ),
  },
  {
    title: 'Cross-platform & tiny',
    icon: '💻',
    description: (
      <>
        One small native Rust binary, no runtime. Runs on Windows, macOS and
        Linux (X11). Press <code>W</code> yourself any time to stop holding.
      </>
    ),
  },
];

function Feature({title, icon, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <span className={styles.featureIcon} role="img" aria-hidden="true">
          {icon}
        </span>
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
