import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.util.*;

public class Menu {


    public static void main(String[] args) throws IOException {
    	final float PRIXGRILLE = 2.5F;
        Scanner entree = new Scanner(System.in);
        String choix;
        //message de mise en garde et demande +18
        Menu.clear();
        Menu.avertissementMenu();

        choix = entree.nextLine();
        //On quite le programme si le message n'est pas accepté
        if(!choix.equalsIgnoreCase("oui") && !choix.equalsIgnoreCase("o")) {
            System.exit(0);
        }

        /// VARIABLES "GLOBALES"
        final int NB_NB_JOUABLES = 50; // pour generer une array de 1 a 50
        int nbGarantis = 2; //garantie que veut l'utilisateur
        float miseMax = 0; //mise max de l'utilisteur
        List<Integer> reducteurNombres = new ArrayList<>(); // nombres que l'utilisateur veut jouer
        List<Integer> nombresDispo = new ArrayList<>(NB_NB_JOUABLES); // nombres que l'utilisateur peut jouer et ajouter a sa liste ci-dessus

        //generation des 50 nombres dispos
        for(int i = 1; i <= NB_NB_JOUABLES; i++) {
            nombresDispo.add(i);
        }
        //System.out.println(nombresDispo);

        //boucle principale
        do {
            //variable de confirmation des actions de l'utilisateur
            Boolean confirm = null;
            //choix sur le menu principal
            switch(choix) {
                case "1":
                    do {

                        //Menu selection actions nombres du réducteur
                        Menu.nombresSystemeMenu(reducteurNombres);
                        System.out.println(">> Saisir une action :");
                        choix = entree.nextLine();

                        //choix ajout ou suppression
                        switch(choix) {
                            case "1":
                                List<String> splitted;
                                List<Integer> toModif;
                                do {
                                    //Menu ajout nombres
                                    ajouterNombresSystemeMenu(reducteurNombres);
                                    if(confirm != null) if(confirm) System.out.println("|| + Nombre(s) ajouté(s)"); else System.out.println("|| /!\\ Nombre(s) non ajouté(s)");
                                    System.out.println(" ");
                                    System.out.println(">> Saisir nombre(s) à ajouter");
                                    System.out.println("séparés par un espace ou une virgule (ou 0 pour quitter) :");
                                    choix = entree.nextLine();
                                    System.out.println(" ");
                                    if(!choix.equals("0")) {
                                        toModif = new ArrayList<>();
                                        confirm = null;
                                        splitted = new ArrayList<String>(Arrays.asList(choix.split(" |,")));
                                        for (String s : splitted) {
                                            try {
                                                Integer olala = Integer.parseInt(s);
                                                if (reducteurNombres.indexOf(olala) == -1 && nombresDispo.indexOf(olala) != -1 && !toModif.contains(olala)) {
                                                    toModif.add(olala);
                                                } else {
                                                    confirm = false;
                                                }
                                            } catch (Exception e) {
                                                confirm = false;
                                            }
                                        }
                                        if (confirm == null) confirm = true;
                                        Collections.sort(toModif);
                                        List<String> filtre = new ArrayList<>();
                                        for (Integer i : toModif) {
                                            filtre.add(String.valueOf(i));
                                        }
                                        splitted.removeAll(filtre);

                                        if(!confirm && toModif.size() > 0) {
                                            System.out.println("|| Ne peuvent pas être ajouté : "+ splitted + " Peuvent être ajouté : " + toModif);
                                            System.out.println("|| Voulez vous ajouter le(s) possible(s) ? (oui/non)");
                                            choix = entree.nextLine();
                                        }
                                        if(choix.equalsIgnoreCase("o") || choix.equalsIgnoreCase("oui") || confirm) {
                                            reducteurNombres.addAll(toModif);
                                            nombresDispo.removeAll(toModif);
                                            Collections.sort(reducteurNombres);
                                            confirm = true;
                                        }
                                        toModif = null;

                                    }

                                } while(!choix.equals("0"));
                                choix = "";
                                confirm = null;
                                break;

                            case "2":
                                do {
                                    //Menu suppression nombres
                                    supprimNombresSystemeMenu(reducteurNombres);
                                    if(confirm != null) if(confirm) System.out.println("|| - Nombre(s) supprimé(s)"); else System.out.println("|| /!\\ Nombre(s) non supprimé(s)");
                                    System.out.println(" ");
                                    System.out.println(">> Saisir nombre(s) à supprimer");
                                    System.out.println("séparés par un espace ou une virgule (ou 0 pour quitter) :");
                                    choix = entree.nextLine();
                                    System.out.println(" ");
                                    if(!choix.equals("0")) {
                                        toModif = new ArrayList<>();
                                        confirm = null;
                                        splitted = new ArrayList<String>(Arrays.asList(choix.split(" |,")));
                                        for (String s : splitted) {
                                            try {
                                                Integer olala = Integer.parseInt(s);
                                                if (reducteurNombres.indexOf(olala) != -1 && nombresDispo.indexOf(olala) == -1 && !toModif.contains(olala)) {
                                                    toModif.add(olala);
                                                } else {
                                                    confirm = false;
                                                }

                                            } catch (Exception e) {
                                                confirm = false;
                                            }
                                        }
                                        if (confirm == null) confirm = true;
                                        Collections.sort(toModif);
                                        List<String> filtre = new ArrayList<>();
                                        for (Integer i : toModif) {
                                            filtre.add(String.valueOf(i));
                                        }
                                        splitted.removeAll(filtre);

                                        if(!confirm && toModif.size() > 0) {
                                            System.out.println("|| Ne peuvent pas être supprimé : "+ splitted + " Peuvent être supprimé : " + toModif);
                                            System.out.println("|| Voulez vous supprimer le(s) possible(s) ? (oui/non)");
                                            choix = entree.nextLine();
                                        }
                                        if(choix.equalsIgnoreCase("o") || choix.equalsIgnoreCase("oui") || confirm) {
                                            reducteurNombres.removeAll(toModif);
                                            nombresDispo.addAll(toModif);
                                            Collections.sort(nombresDispo);
                                            confirm = true;
                                        }
                                        toModif = null;

                                    }
                                } while(!choix.equals("0"));
                                choix = "";
                                confirm = null;
                                break;
                        }

                    } while(!choix.equals("0"));
                    break;

                case "2":
                    confirm = null;
                    do {
                        //menu garantie
                        Menu.garantieMenu(nbGarantis);
                        if(confirm != null) if(confirm) System.out.println("|| # Garantie modifiée"); else System.out.println("|| /!\\ Nombre invalide");
                        System.out.println(" ");
                        System.out.println(">> Saisir la grantie désirée (ou 0 pour quitter) :");
                        choix = entree.nextLine();
                        if(!choix.equals("0")) {
                            try {
                                Integer nb = Integer.valueOf(choix);
                                if(confirm = (nb > 1 && nb < 5)) nbGarantis = nb;
                            } catch (Exception e) {
                                confirm = false;
                            }
                        }

                    } while(!choix.equals("0"));
                    break;

                case "3":
                    confirm = null;
                    do {
                        //menu mise max
                        Menu.miseMaxMenu(miseMax);
                        if(confirm != null) if(confirm) System.out.println("|| # Mise max modifiée"); else System.out.println("|| /!\\ Nombre invalide");
                        System.out.println(" ");
                        System.out.println(">> Saisir la mise max (ou 0 pour quitter) :");
                        choix = entree.nextLine();
                        if(!choix.equals("0")) {
                            float nb;
                            try {
                                nb = Float.valueOf(choix);
                                if(confirm = (nb > 0.0f && nb <= 10000.0f)) miseMax = nb;
                            } catch (Exception e) {
                                confirm = false;
                            }
                        }

                    } while(!choix.equals("0"));
                    break;

                case "4":
                    do {
                        Menu.clear();
                        if(reducteurNombres.size() < 11) {
                            System.out.println("=================================");
                            System.out.println("/!\\ Impossible de générer les grilles, vous n'avez pas saisi assez de nombres.");
                            System.out.println("=================================");
                        }
                        final int TAILLESYSREDUC = reducteurNombres.size();
                		final int GARANTIS = nbGarantis;
                		
                		//Code permettant de récupérer le chemin du bon fhichier
                		String nFicLoad = "Ressources/data_";
                		nFicLoad+=TAILLESYSREDUC+"_"+GARANTIS+".txt";
                		Path path = Paths.get(nFicLoad);
                		BufferedReader monFic = null;;
                		try {
                			monFic = Files.newBufferedReader(path, Charset.forName("UTF-8"));
                		} catch (IOException e) {
                			e.printStackTrace();
                		}
                		
                		// code permettant de transformer les infos du fichier en objets de type grille :
                		List<Grille> grillesRecuperees = new ArrayList<>();
                		String ligne = "";
                		try {
                			ligne = monFic.readLine();
                		} catch (IOException e) {
                			e.printStackTrace();
                		}
                		while (ligne != null) {
                			grillesRecuperees.add(new Grille(ligne));
                			try {
                				ligne = monFic.readLine();
                			} catch (IOException e) {
                				e.printStackTrace();
                			}
                		}
                		
                		//Calcul du prix des grilles
                		float prix = grillesRecuperees.size()*PRIXGRILLE;
                		
                		choix = "oui";
                		//Vérification du respect de la mise maximum, avec une marge de 15%
                		if (prix > miseMax * 1.15F) {
                			Menu.prixDepasseMiseMenu(prix, miseMax);
                	        choix = entree.nextLine();
                		}
                		
                		if (choix.toLowerCase().equals("oui")) {
							//On fait la correspondance
							Outils.convertir(reducteurNombres, grillesRecuperees);
							//Trouver le chemin du fichier ou on va stoquer les résultats, le fichier est cree a partir de la date courante
							DateFormat format = new SimpleDateFormat("yyyy_MM_dd-HH_mm_ss");
							Date dateCourante = new Date();
							String nFic = "Resultats/";
							nFic += format.format(dateCourante) + ".txt";
							File f = new File(nFic);
							try {
								f.createNewFile();
							} catch (IOException e1) {
								e1.printStackTrace();
							}
							//Enregistrement des grilles
							List<String> strGrilles = new ArrayList<>();
							for (int i = 0; i < grillesRecuperees.size(); i++) {
								if (i > 0 && i % 5 == 0)
									strGrilles.add("");
								strGrilles.add(grillesRecuperees.get(i).toString());
							}
							Path fichier = Paths.get(nFic);
							try {
								Files.write(fichier, strGrilles, Charset.forName("UTF-8"), StandardOpenOption.APPEND);
							} catch (IOException e) {
								e.printStackTrace();
							}
							//Affichahe des grilles jouées
							//System.out.println( grillesRecuperees.size()+" grilles récupérées : \n"+grillesRecuperees);
							Menu.clear();
							System.out.println("\n" + grillesRecuperees.size() + " grilles générées dans le fichier \""
									+ nFic + "\" : \n" + grillesRecuperees);
							System.out.println("\n MONTANT A MISER :" + prix + "€\n");
							//Possibilite de retour au menu principal
	                		System.out.println("\n0 >> Revenir au menu");
	                        System.out.println(" ");
	                        choix = entree.nextLine();
                		} else if (choix.toLowerCase().equals("non")){
                			choix = "0";
                		}                		
                    } while(!choix.equals("0"));
                    break;
            }

            //on affiche le menu principal
            Menu.accueilMenu(reducteurNombres.size(), nbGarantis, miseMax);
            System.out.println(">> Saisir une action :");
            choix = entree.nextLine();

        } while (!choix.equals("0"));
        entree.close();

    }

    /**
     * Vide l'affichage console
     */
    private static void clear() {
        //methode dégeux de clear
        for (int y = 0; y < 50; y++)
           System.out.println("\n");
    }
    
    /**
     * Affichage de l'avertissement
     */
    private static void avertissementMenu() {
        System.out.println("/!\\ Attention /!\\");
        System.out.println(" ");
        System.out.println("Jouer comporte des risques : isolement, endettement...");
        System.out.println("Appelez le 09-74-75-13-13 (appel non surtaxé).");
        System.out.println(" ");
        System.out.println("Les jeux d'argents sont interdits aux mineurs,");
        System.out.println("en continuant vous confirmez avoir plus de 18 ans.");
        System.out.println(" ");
        System.out.println("Confirmez vous avoir plus de 18 ans ? (oui/non)");
    }

    /**
     * Affichage du menu d'accueil
     * @param taillesys taille du systeme reducteur
     * @param garanti nombre de nombres garantis
     * @param mm Mise maximale
     */
    private static void accueilMenu(int taillesys, int garanti, float mm) {
        Menu.clear();
        System.out.println("=================================");
        System.out.println("||        MENU PRINCIPAL       ||");
        System.out.println("=================================");
        System.out.println("|| # " + taillesys + " nombre(s) choisi(s).");
        System.out.println("|| # " + garanti + " nombre(s) garanti(s).");
        System.out.println("|| # Mise maximale définie : " + mm);
        System.out.println("||");
        System.out.println("|| 1 >> Choix des nombres du systeme");
        System.out.println("|| 2 >> Choix garantie");
        System.out.println("|| 3 >> Choix mise max");
        System.out.println("||");
        System.out.println("|| 4 >> Générer les grilles");
        Menu.finMenu();
    }

    /**
     * Affichage du menu du choix des nombres du système réducteur
     * @param r liste des nombres choisis
     */
    private static void nombresSystemeMenu(List<Integer> r) {
        Menu.clear();
        System.out.println("=================================");
        System.out.println("||      Nombres du systeme     ||");
        System.out.println("=================================");
        System.out.println("|| # Vous avez choisi " + r.size() + " nombre(s).");
        System.out.println("|| # Qui sont : " + r);
        System.out.println("||");
        System.out.println("|| 1 >> Ajouter des nombres");
        System.out.println("|| 2 >> Supprimer des nombres");
        Menu.finMenu();
    } 

    /**
     * Affichage du menu de supression de nombres
     * @param r	liste des nombres choisis
     */
    private static void ajouterNombresSystemeMenu(List<Integer> r) {
        Menu.clear();
        System.out.println("=================================");
        System.out.println("||       Ajouter nombres       ||");
        System.out.println("=================================");
        System.out.println("|| # Les nombres sont compris entre 1 et 50.");
        System.out.println("|| # Vous avez choisi " + r.size() + " nombre(s).");
        System.out.println("|| # Qui sont : " + r);
        System.out.println("||");
    }

    /**
     * Affichage du menu de supression de nombres
     * @param r liste des nombres choisis
     */
    private static void supprimNombresSystemeMenu(List<Integer> r) {
        Menu.clear();
        System.out.println("=================================");
        System.out.println("||      Supprimer nombres      ||");
        System.out.println("=================================");
        System.out.println("|| # Vous avez choisi " + r.size() + " nombre(s).");
        System.out.println("|| # Qui sont : " + r);
        System.out.println("||");
    }

    /**
     * Affichage du menu du choix du nombres de nombres garantis
     * @param g nombre de nombres garantis
     */
    private static void garantieMenu(int g) {
        Menu.clear();
        System.out.println("=================================");
        System.out.println("||       Nombres garantis      ||");
        System.out.println("=================================");
        System.out.println("|| # Vous aurez " + g + " nombre(s) garanti(s).");
        System.out.println("|| # Garantie comprise entre 2 et 4.");
        System.out.println("||");
    }

    /**
     * Affichage du menu de mise maximum
     * @param mm mise maximale
     */
    private static void miseMaxMenu(float mm) {
        Menu.clear();
        System.out.println("=================================");
        System.out.println("||           Mise max          ||");
        System.out.println("=================================");
        System.out.println("|| # Mise maximale définie : " + mm);
        System.out.println("|| # Max : 10 000euros.");
        System.out.println("||");
    }

    /**
     * Affichage de la fin commune à plusieurs menus
     */
    private static void finMenu() {
        System.out.println("||");
        System.out.println("|| 0 >> Quitter");
        System.out.println("=================================");
        System.out.println(" ");
    }
    
    private static void prixDepasseMiseMenu(float p, float mm) {
        System.out.println("=================================");
		System.out.println("||     /!\\ Attention /!\\     ||");
        System.out.println("=================================");
        System.out.println("||");
        System.out.println("|| Avec les paramètres sélectionnés, la somme");
        System.out.println("|| que vous devez miser dépasse de plus de 15%");
        System.out.println("|| la mise maximale que vous aviez indiqué.");
        System.out.println("|| Souhaitez vous continuer ?");
        System.out.println("|| Prix extimé : "+p+"€");
        System.out.println("|| Mise maximale saisie : "+mm+"€");
        System.out.println("|| ");
        System.out.println("|| oui : continuer");
        System.out.println("|| non : retour au menu");
    }


//    private static void modifNombres(Boolean confirm, List<String> splitted, String choix, List<Integer> reducteurNombres, List<Integer> nombresDispo, Scanner entree) {
//        List<Integer> toModif = new ArrayList<>();
//        confirm = null;
//        splitted = new ArrayList(Arrays.asList(choix.split(" |,")));
//        for(int i = 0; i < splitted.size(); i ++) {
//            try {
//                Integer olala = Integer.parseInt(splitted.get(i));
//                if(reducteurNombres.indexOf(olala) == -1 && nombresDispo.indexOf(olala) != -1 && !toModif.contains(olala)) {
//                    toModif.add(olala);
//                } else {
//                    confirm = false;
//                }
//                if(confirm == null) confirm = true;
//            } catch (Exception e) {
//                confirm = false;
//            }
//        }
//
//        Collections.sort(toModif);
//        List<String> filtre = new ArrayList<>();
//        for (Integer i : toModif) {
//            filtre.add(String.valueOf(i));
//        }
//        splitted.removeAll(filtre);
//
//        if(!confirm && toModif.size() > 0) {
//            System.out.println("||");
//            System.out.println("|| Ne peuvent pas être ajoute : "+ splitted + " Peuvent être ajoute : " + toModif);
//            System.out.println("|| Voulez vous ajouter le(s) possible(s) ? (oui/non)");
//            choix = entree.nextLine();
//        }
//        if(choix.equals("o") || choix.equals("oui") || confirm) {
//            reducteurNombres.addAll(toModif);
//            nombresDispo.removeAll(toModif);
//            Collections.sort(reducteurNombres);
//            confirm = true;
//        }
//    }


}
