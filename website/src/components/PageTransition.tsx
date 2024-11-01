import { motion } from 'framer-motion';
import { ReactNode } from 'react';

interface PageTransitionProps {
  children: ReactNode;
}

const PageTransition: React.FC<PageTransitionProps> = ({ children }) => {
  
  const getTransitionProps = () => {
    // Default transition
    const defaultTransition = {
      initial: { opacity: 0, y: 0 },
      animate: { opacity: 1, y: 0 },
      exit: { opacity: 1, y: 0 },
      transition: { duration: 0.2 }
    };

    return defaultTransition;
  };

  const transitionProps = getTransitionProps();

  return (
    <motion.div
      {...transitionProps}
    >
      {children}
    </motion.div>
  );
};

export default PageTransition; 