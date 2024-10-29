import React from 'react';

const AnimatedBackground: React.FC = () => {
  const backgroundText = `My father's family name being Pirrip, and my Christian name Philip, my infant tongue could make of both names nothing longer or more explicit than Pip. So, I called myself Pip, and came to be called Pip. I give Pirrip as my father's family name, on the authority of his tombstone and my sister,—Mrs. Joe Gargery, who married the blacksmith. As I never saw my father or my mother, and never saw any likeness of either of them (for their days were long before the days of photographs), my first fancies regarding what they were like were unreasonably derived from their tombstones. The shape of the letters on my father's, gave me an odd idea that he was a square, stout, dark man, with curly black hair. From the character and turn of the inscription, "Also Georgiana Wife of the Above," I drew a childish conclusion that my mother was freckled and sickly. Ours was the marsh country, down by the river, within, as the river wound, twenty miles of the sea. My first most vivid and broad impression of the identity of things seems to me to have been gained on a memorable raw afternoon towards evening. `.repeat(20);

  return (
    <div className="fixed top-0 left-0 -z-10 w-screen h-screen overflow-hidden select-none pointer-events-none bg-gray-900">
      <div className="animate-scroll whitespace-pre-wrap text-gray-100/5 text-xs leading-tight p-4">
        {backgroundText}
        {backgroundText}
      </div>
    </div>
  );
};

export default AnimatedBackground;
