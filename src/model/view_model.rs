pub trait BaseSubjectType {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActivityState {
    Inactive,
    Loading,
    Active,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InnerModel<T> {
    State: ActivityState,
    Subject: Option<T>,
}

impl<T> InnerModel<T> {
    fn new() -> InnerModel<T> {
        InnerModel {
          State: ActivityState::Inactive,
          Subject: None,
        }
    }

    fn InnerActivate(&mut self) {
        unimplemented!();
    }

    fn InnerDeactivate(&mut self) {
        unimplemented!();
    }

    fn InnerLoading(&mut self) {
        unimplemented!();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ViewModel<T> {
    State: ActivityState,
    Subject: Option<T>,
    Inner: Option<InnerModel<T>>,
}

impl<T> ViewModel<T> {
    pub fn new(inner_model: Option<InnerModel<T>>) -> ViewModel<T> {
        ViewModel {
            Subject: None,
            State: ActivityState::Inactive,
            Inner: inner_model,
        }
    }
}

pub trait IModel {
    type SubjectType;
    fn change_model(&mut self, new_model: Option<InnerModel<Self::SubjectType>>);
    fn change_subject(&mut self, subject: Option<Self::SubjectType>);
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn loading(&mut self);
}

pub trait StateTransition {
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn loading(&mut self);
}


impl<T: PartialEq> IModel for ViewModel<T> {
    type SubjectType = T;

    fn change_model(&mut self, new_model: Option<InnerModel<Self::SubjectType>>) {
        if self.Subject.is_none() {return;}

        if self.Inner != new_model || self.State == ActivityState::Loading {
            let previous_state = self.State.clone();

            // Temporarily disable the models changes
            match self.State {
                ActivityState::Active | ActivityState::Inactive => {
                    self.loading();
                }
                _ => {}
            }

            self.Inner = new_model;

            // re-set the models changes
            match previous_state {
                ActivityState::Active => {
                    self.activate();
                }
                ActivityState::Inactive=> {
                    self.deactivate();
                }
                _ => {}
            }
        }
    }

    fn change_subject(&mut self, subject: Option<Self::SubjectType>) {
        if self.Subject != subject || self.State == ActivityState::Loading {
            let was_active = self.State.clone();

            // Temporarily disable the models changes
            match self.State {
                ActivityState::Active => {
                    self.deactivate();
                    match self.State {
                        ActivityState::Inactive => {
                            self.loading();
                        }
                        _ => {
                            print!("Deactivating failed for:");
                        }
                    }
                }
                _ => {}
            }

            self.Subject = subject;

            // re-enable the models changes
            match was_active {
                ActivityState::Active | ActivityState::Loading => {
                    self.activate();
                }
                _ => {}
            }
        }
    }

    fn activate(&mut self) {
        // Idea - if its inactive, auto mark it as loading, then active?
        // match self.State {
        //     ActivityState::Inactive => {
        //       // Mark the Model as Active
        //       self.State = ActivityState::Loading;

        //       if self.Inner.is_some()
        //       {
        //         let mut inner_model = self.Inner.take().unwrap();

        //         inner_model.InnerLoading();

        //         self.Inner = Some(inner_model);
        //       }

        //       self.activate();
        //     }
        //     ActivityState::Loading => {
        //       // Mark the Model as Active
        //       self.State = ActivityState::Active;

        //       if self.Inner.is_some()
        //       {
        //         let mut inner_model = self.Inner.take().unwrap();

        //         inner_model.InnerActivate();

        //         self.Inner = Some(inner_model);
        //       }
        //     }
        // }

        if self.State == ActivityState::Inactive || self.State == ActivityState::Loading {
            // Mark the Model as Active
            self.State = ActivityState::Active;

            if self.Inner.is_some() {

                let mut inner_model = self.Inner.take().unwrap();

                inner_model.InnerActivate();

                self.Inner = Some(inner_model);

            }

        }
    }

    fn deactivate(&mut self) {
        if self.State == ActivityState::Active || self.State == ActivityState::Loading {

            // Mark the Model as Inactive
            self.State = ActivityState::Inactive;

            let mut inner_model = self.Inner.take().unwrap();

            inner_model.InnerDeactivate();

            self.Inner = Some(inner_model);

        }
    }

    fn loading(&mut self) {
        if self.State == ActivityState::Active || self.State == ActivityState::Inactive {

            // Mark the Model as Inactive
            self.State = ActivityState::Loading;

            let mut inner_model = self.Inner.take().unwrap();

            inner_model.InnerLoading();

            self.Inner = Some(inner_model);

        }
    }
}

impl<T: PartialEq> StateTransition for InnerModel<T> {
    fn activate(&mut self) {
        self.InnerActivate();
    }

    fn deactivate(&mut self) {
        self.InnerDeactivate();
    }

    fn loading(&mut self) {
        self.InnerLoading();
    }
}

#[cfg(test)]
mod tests {
    use super::{ActivityState, IModel, InnerModel, ViewModel};
    use model::map::{Map, MapBuilder};

    #[test]
    fn no_subject() {
        let elem: ViewModel<Map> = ViewModel::new(None);

        assert!({
            match elem.Subject {
                None => true,
                _ => false,
            }
        });
    }

    #[test]
    fn subject_not_changing() {
        let mut elem: ViewModel<Map> = ViewModel::new(None);

        elem.change_subject(None);

        assert!({
            match elem.Subject {
                None => true,
                _ => false,
            }
        });

        match MapBuilder::from_named(None) {
            Ok(map) => elem.change_subject(Some(map)),
            Err(ex) => panic!(ex),
        };

        assert!({
            let name_test = match elem.clone().Subject {
                None => false,
                Some(map) => map.name() == "Tutorial",
            };

            let subject_test = match elem.State {
                ActivityState::Inactive => true,
                _ => false,
            };

            name_test && subject_test
        });
    }

    #[test]
    fn subject_activating() {
        use self::ActivityState::*;

        let mut elem: ViewModel<Map> = ViewModel::new(None);

        // Fails, need to implement Activate...
        elem.activate();

        assert!(match elem.State {
            Active => true,
            _ => false,
        });
    }

    #[test]
    fn change_model_no_subject() {
        let mut elem: ViewModel<Map> = ViewModel::new(None);

        elem.change_model(Some(InnerModel::new()));

        // Model shouldn't be set, as there is no subject
        assert!(match elem.Inner {
            None => true,
            Some(_) => false,
        });
    }
}